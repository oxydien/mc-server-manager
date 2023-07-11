<style lang="scss" scoped>
@import "../../assets/scss/global.scss";

.file-grid {
  h2 {
    position: sticky;
    top: -5px;
    z-index: 68;
    background-color: var(--color-bg);
    padding: 5px;
    .separator {
      margin-left: 3px;
    }

    span {
      cursor: pointer;
    }
  }
  .file-wrapper {
    display: flex;
    gap: 10px;

    &.grid {
      justify-content: stretch;
      flex-flow: row wrap;
      justify-content: baseline;
      align-items: stretch;
    }
    &.list {
      flex-flow: column nowrap;

      .file-box {
        flex-direction: row;
        width: 100%;

        img {
          max-height: 80px;
        }
      }
    }

    .file-box {
      display: flex;
      gap: 10px;
      align-items: center;
      flex-direction: column;
      width: 120px;
      text-align: center;

      word-break: break-all;
      overflow-wrap: break-word;

      background-color: $fg-a;
      padding: 10px;
      box-sizing: border-box;
      border-radius: 10px;
      cursor: default;
      position: relative;

      .file-name {
        text-align: start;
        width: 100%;
      }

      img {
        aspect-ratio: 1/1;
        max-height: 120px;
        max-width: 100%;
      }

      .additional-info {
        display: flex;
        align-items: center;
        gap: 10px;
        .time {
          display: flex;
          gap: 10px;
          font-size: 0.8rem;
          margin-top: 5px;
          flex-flow: nowrap column;
          align-items: flex-start;
          min-width: 150px;
        }
        .size {
          min-width: 100px;
          text-align: end;
        }
      }
    }
    .can-open {
      cursor: pointer;
    }
  }
}
</style>

<template>
  <div class="file-grid" ref="fileGrid" v-show="!fileOpen">
    <h2>
      <Button @click="changeViewType" iconOnly style="float: right">
        <GridIcon v-if="viewType === 'list'" />
        <ListIcon v-else />
      </Button>
      <Button
        iconOnly
        style="display: inline-block;"
        :action="
          () => {
            this.path = '';
            this.goToFolder();
          }
        "
      ><HomeIcon /></Button>
      <span
        v-for="(pathPart, index) in path.split('\\')"
        @click="getPathUpToFolder(pathPart)"
        :key="index"
        ><span class="separator">/</span>{{ pathPart }}
      </span>
    </h2>
    <div class="file-wrapper" :class="viewType" :key="serverFiles">
      <Button
        v-if="path != ''"
        class="file-box directory can-open"
        @click="goToParentFolder"
      >
        <img src="/file-icons/folder-back.svg" />
        ../
      </Button>
      <Button
        v-for="(file, index) in serverFiles"
        :key="index"
        class="file-box"
        :class="{
          directory: file.file_type === 'directory',
          canOpen: isFileReadable(file.file_type),
        }"
        :style="{
          cursor:
            isFileReadable(file.file_type) || file.file_type === 'directory'
              ? 'pointer'
              : 'default',
        }"
        @click="handleClick(file)"
      >
        <img
          :src="
            file.file_type === 'directory'
              ? '/file-icons/' + (svgIcons[file.name] || 'folder-null') + '.svg'
              : '/file-icons/' +
                (svgIcons[file.file_type] || 'file-null') +
                '.svg'
          "
        />
        <span class="file-name">{{ file.name }}</span>
        <div class="additional-info" v-if="viewType === 'list'">
          <div class="time">
            <span>{{ parseDateTime(file.created) }}</span>
            <span>{{ parseDateTime(file.modified) }}</span>
          </div>
          <span v-if="file.file_type !== 'directory'" class="size">{{
            formatBytes(file.size_bytes)
          }}</span>
        </div>
      </Button>
    </div>
  </div>
  <div class="file-textarea" v-if="fileOpen">
    <TextEditor
      :fileName="fileName"
      :fileType="fileType"
      :fileContent="fileContent"
      :filePath="filePath"
      :serverId="serverId"
      @updatedContent:fileContent="(newContent) => (fileContent = newContent)"
      @CloseTextEditor="fileOpen = false"
    >
    </TextEditor>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import TextEditor from "./textEditor.vue";
import { Button, Modal, Chips, ListIcon, GridIcon, HomeIcon } from "omorphia";

export default {
  name: "FileSystem",
  data() {
    return {
      files: [],
      path: "",
      serverFiles: [],
      viewType: "list",
      readableFiles: ["log", "json", "properties", "txt", "conf", "settings", "toml", "yaml", "md"],
      svgIcons: {
        log: "file-txt",
        txt: "file-txt",
        md: "file-txt",
        json: "file-properties",
        toml: "file-properties",
        yaml: "file-properties",
        properties: "file-properties",
        settings: "file-properties",
        conf: "file-properties",
        zip: "file-zip",
        jar: "file-zip",
        gz: "file-zip",
        world: "folder-world",
        mods: "folder-mods",
        logs: "folder-logs",
        config: "folder-config",
      },
      fileOpen: false,
      fileContent: "",
      fileType: "",
      fileName: "",
      filePath: "",
    };
  },
  components: {
    TextEditor,
    Button,
    Modal,
    Chips,
    ListIcon,
    GridIcon,
    HomeIcon,
  },
  props: {
    serverId: {
      default: "",
      type: String,
    },
  },
  mounted() {
    this.goToFolder();
  },
  methods: {
    goToFolder(name = null) {
      if (name != null) {
        if (this.path == "") {
          this.path = name;
        } else {
          this.path += "\\" + name;
        }
      }
      this.files = [];
      this.getFiles(this.path).then(() => {
        this.sortedFiles();
      });
    },
    goToParentFolder() {
      const pathArray = this.path.split("\\");
      pathArray.pop();
      this.path = pathArray.join("\\");
      this.goToFolder();
    },
    handleClick(file) {
      if (this.isFileReadable(file.file_type)) {
        this.getFileContents(file.name, file.file_type, file.name);
      } else if (file.file_type === "directory") {
        this.goToFolder(file.name);
      }
    },
    async getFileContents(file, fileType, fileName) {
      this.fileType = fileType;
      this.fileName = fileName;
      this.filePath =
        this.path == "" ? this.path + file : this.path + "\\" + file;
      this.fileContent = await invoke("get_server_file", {
        filePath: this.filePath,
        serverId: this.serverId,
      });
      this.fileOpen = true;
    },
    getPathUpToFolder(folderName) {
      const folders = this.path.split("\\");
      const folderIndex = folders.indexOf(folderName);
      if (folderIndex !== -1) {
        const pathUpToFolder = folders.slice(0, folderIndex + 1);
        this.path = pathUpToFolder.join("\\");
      } else {
        this.path = "";
      }
      this.goToFolder();
    },
    async getFiles(path = "") {
      if (this.serverId != "") {
        this.files = JSON.parse(
          await invoke("get_files_command", {
            serverId: this.serverId,
            path: path,
          })
        );
      }
    },
    sortedFiles() {
      const directories = this.files.filter(
        (file) => file.file_type === "directory"
      );
      const files = this.files.filter((file) => file.file_type !== "directory");
      this.serverFiles = [...directories, ...files];
    },
    isFileReadable(fileType) {
      return this.readableFiles.some((file) => fileType.includes(file));
    },
    changeViewType() {
      this.viewType = this.viewType === "list" ? "grid" : "list";
    },
    formatBytes(bytes) {
      if (bytes === 0) return "0 Bytes";

      const k = 1024;
      const sizes = ["Bytes", "KB", "MB", "GB"];

      const i = Math.floor(Math.log(bytes) / Math.log(k));
      const formattedSize = parseFloat((bytes / Math.pow(k, i)).toFixed(2));

      return `${formattedSize} ${sizes[i]}`;
    },
    parseDateTime(dateTimeString) {
      const dateTime = new Date(dateTimeString);

      const year = dateTime.getFullYear();
      const month = dateTime.toLocaleString("default", { month: "long" });
      const day = dateTime.getDate();

      const hours = dateTime.getHours();
      const minutes = dateTime.getMinutes();
      const seconds = dateTime.getSeconds();

      return `${month} ${day}, ${year} ${hours}:${minutes}:${seconds}`;
    },
  },
};
</script>

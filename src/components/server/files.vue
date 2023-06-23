<style lang="scss" scoped>
@import "../../assets/scss/global.scss";

.file-grid {
  h2 {
    margin: 5px;
    .separator {
      margin-left: 3px;
    }

    span {
      cursor: pointer;
    }
  }
  .file-wrapper {
    display: flex;
    flex-wrap: wrap;
    justify-content: baseline;
    align-items: stretch;
    gap: 10px;

    .file-box div {
      --_size: 1;
      display: flex;
      gap: 10px;
      align-items: center;
      flex-direction: column;
      font-size: max(calc(0.5rem * (var(--_size))), 1.2rem);
      width: calc(80px * var(--_size));
      height: 100%;
      text-align: center;

      word-break: break-all;
      overflow-wrap: break-word;

      background-color: $fg-a;
      padding: 10px;
      box-sizing: border-box;
      border-radius: 10px;

      img {
        width: calc(50px * var(--_size));
        height: calc(50px * var(--_size));
      }
      &.directory img {
        width: calc(70px * var(--_size));
        height: calc(70px * var(--_size));
      }
    }
    .can-open {
      cursor: pointer;
    }
  }
}
</style>

<template>
  <div class="file-grid" v-show="!fileOpen">
    <h2>
      <input
        type="range"
        step="0.001"
        style="float: right"
        min="1"
        max="5"
        v-model="folderScale"
      />
      <span
        v-for="(pathPart, index) in path.split('\\')"
        @click="getPathUpToFolder(pathPart)"
        :key="index"
        ><span class="separator">/</span>{{ pathPart }}
      </span>
    </h2>
    <div class="file-wrapper" :key="serverFiles">
      <div v-if="path != ''" class="file-box">
        <div
          class="directory can-open"
          @click="goToParentFolder"
          :style="'--_size:' + folderScale"
        >
          <img src="/file-icons/folder-back.svg" />
          ../
        </div>
      </div>
      <div v-for="(file, index) in serverFiles" :key="index" class="file-box">
        <div
          v-if="file.file_type === 'directory'"
          @click="go_to_folder(file.name)"
          class="directory can-open"
          :data-file-type="file.file_type"
          :style="'--_size:' + folderScale"
        >
          <img
            :src="
              '/file-icons/' + (svgIcons[file.name] || 'folder-null') + '.svg'
            "
          />
          <span>{{ file.name }}</span>
        </div>
        <div
          v-else
          class="file"
          @click="
            () => {
              if (isFileReadable(file.file_type)) {
                getFileContents(file.name, file.file_type, file.name);
              }
            }
          "
          :class="{ 'can-open': isFileReadable(file.file_type) }"
          :style="'--_size:' + folderScale"
        >
          <img
            :src="
              '/file-icons/' +
              (svgIcons[file.file_type] || 'file-null') +
              '.svg'
            "
          />
          <span>{{ file.name }}</span>
        </div>
      </div>
    </div>
  </div>
  <div class="file-textarea" v-if="fileOpen">
    <TextEditor
      :fileName="fileName"
      :fileType="fileType"
      :fileContent="fileContent"
      :filePath="filePath"
      :serverId="serverId"
      @updatedContent:fileContent="newContent => fileContent = newContent"
      @CloseTextEditor="fileOpen = false"
    >
    </TextEditor>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import TextEditor from "./textEditor.vue";

export default {
  name: "FileSystem",
  data() {
    return {
      files: [],
      path: "",
      serverFiles: [],
      folderScale: 1.962,
      readableFiles: ["log", "json", "properties", "txt"],
      svgIcons: {
        log: "file-txt",
        txt: "file-txt",
        json: "file-properties",
        properties: "file-properties",
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
    go_to_folder(name) {
      if (this.path == "") {
        this.path = name;
      } else {
        this.path += "\\" + name;
      }
      this.goToFolder();
    },
    goToParentFolder() {
      const pathArray = this.path.split("\\");
      pathArray.pop();
      this.path = pathArray.join("\\");
      this.goToFolder();
    },
    goToFolder() {
      this.get_files(this.path).then(() => {
        this.sortedFiles();
      });
    },
    isFileReadable(fileType) {
      return this.readableFiles.some((file) => fileType.includes(file));
    },
    async getFileContents(file, fileType, fileName) {
      this.fileType = fileType;
      this.fileName = fileName;
      this.filePath = this.path == "" ? this.path + file : this.path + "\\" + file;
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
    async get_files(path = "") {
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
  },
};
</script>

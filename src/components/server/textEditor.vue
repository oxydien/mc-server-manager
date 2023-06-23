<style lang="scss" scoped>
@import "../../assets/scss/global.scss";
$_padding: 10px;

.text-editor {
  background: $fg-a;
  padding: $_padding;
  box-sizing: border-box;
  border-radius: $brr;

  h2 {
    margin: 2px;
    .flex-heading {
      display: flex;
      gap: 5px;
      align-items: center;
      margin: 0;
      border-bottom: 1px solid $fg-h;

      * {
        margin: 0;
      }
      svg {
        cursor: pointer;
      }
    }
    .open-button {
      float: right;
      display: flex;
      align-items: center;
      font-size: 1.2rem;
      background-color: $fg-h;
      color: $txt-m;
      margin-top: -5px;
      transition: all 50ms;
      padding: 3px 8px;
      border-radius: 8px;
      border: none;
      cursor: pointer;

      &:active,
      &:hover {
        background-color: $main-am;
      }
      p {
        margin: 0;
      }
    }
    .save-button {
      float: right;
      font-size: 1.2rem;
      background-color: $fg-h;
      margin-right: 5px;
      margin-top: -5px;
      transition: all 50ms;
      padding: 0 8px;
      border-radius: 8px;
      border: none;
      cursor: pointer;
      width: 30px;
      height: 30px;

      &:active,
      &:hover {
        background-color: $main-am;
      }
      p {
        margin: 0;
      }
    }
  }
}

.text-holder {
  display: block;
  width: 100%;
  height: calc(100vh - 120px);
  overflow-y: auto;
  margin: 0 auto;
  position: relative;
}

.text,
.write {
  position: absolute;
  top: $_padding;
  border: none;
  padding: 0;
  outline: none;
  resize: none;
  width: calc(100% - 20px);
  height: 100000vh;
  display: block;
  font-size: 1.1rem;
  line-height: 1;
  text-rendering: geometricPrecision;
  white-space: pre-wrap !important;
  word-break: break-all !important;
  overflow-wrap: break-word !important;
  overflow: hidden;
  font-family: "Courier New", Courier, monospace;
  box-sizing: border-box;
  hyphens: auto;
}

.write {
  background: transparent;
  color: #8b8b8b36;
}

.save-button {
  background-image: url(/file-icons/save-icon.svg);
  background-repeat: no-repeat;
  background-position: 50% 50%;

  &.success {
    background-image: url(/file-icons/save-icon-success.svg);
    animation: 500ms successAnimation forwards ease-in-out;
  }
  &.fail {
    background-image: url(/file-icons/save-icon-fail.svg);
  }

  @keyframes successAnimation {
    0% {
      background-position: 50% 50%;
    }
    10% {
      background-position: 50% 80%;
    }
    80% {
      background-position: 50% 10%;
    }
    100% {
      background-position: 50% 50%;
    }
  }
}
</style>

<template>
  <div class="text-editor">
    <h2>
      <button class="open-button" @click="openInFiles">
        <p>Open file</p>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        >
          <path
            fill="none"
            stroke="currentColor"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M10 4H6a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-4m-8-2l8-8m0 0v5m0-5h-5"
          />
        </svg>
      </button>
      <button class="save-button" ref="savingFile" @click="setFileContent">

      </button>
      <div class="flex-heading">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 512 512"
          @click="closeTextEditor"
        >
          <path
            fill="currentColor"
            d="M321.94 98L158.82 237.78a24 24 0 0 0 0 36.44L321.94 414c15.57 13.34 39.62 2.28 39.62-18.22v-279.6c0-20.5-24.05-31.56-39.62-18.18Z"
          />
        </svg>
        <p>{{ fileName }}</p>
      </div>
    </h2>
    <div class="text-holder">
      <div class="text" v-html="highlight"></div>
      <textarea
        spellcheck="false"
        class="write"
        v-model="editableContent"
      ></textarea>
    </div>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
export default {
  name: "TextEditor",
  props: {
    serverId: {
      default: "",
      type: String,
    },
    fileContent: {
      default: "",
      type: String,
    },
    fileType: {
      default: "",
      type: String,
    },
    fileName: {
      default: "",
      type: String,
    },
    filePath: {
      default: "",
      type: String,
    },
  },
  data() {
    return {
      editableContent: "",
    };
  },
  mounted() {
    this.editableContent = this.fileContent;
  },
  watch: {
    editableContent(newContent) {
      this.$emit("updatedContent", newContent);
    },
  },
  methods: {
    closeTextEditor() {
      this.$emit("CloseTextEditor");
    },
    async openInFiles() {
      console.log(
        await invoke("open_file_or_explorer", {
          serverId: this.serverId,
          path: this.filePath,
        })
      );
    },
    async setFileContent() {
      if (
        await invoke("set_server_file", {
          filePath: this.filePath,
          contents: this.editableContent,
          serverId: this.serverId,
        })
      == null) {
        this.$refs["savingFile"].classList.add("success");
        setTimeout(() => {
          this.$refs["savingFile"].classList.remove("success");
        }, 2000);
      }
      else {
        this.$refs["savingFile"].classList.add("fail");
        setTimeout(() => {
          this.$refs["savingFile"].classList.remove("fail");
        }, 2000);
      }
    },
  },
  computed: {
    highlight() {
      switch (this.fileType) {
        case "json": {
          return this.highlightedJson;
        }
        case "log": {
          return this.highlightedLog;
        }
        case "properties": {
          return this.highlightedProperties;
        }
        case "txt": {
          return this.editableContent;
        }
      }
    },
    highlightedLog() {
      const logText = this.editableContent
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;");
      const regexes = [
        { pattern: /"\b[^"]*\b"/g, class: "#32daf5" },
        { pattern: /\b[0-9]{2}:[0-9]{2}:[0-9]{2}\b/g, class: "#aaa" },
        { pattern: /\bINFO\b/g, class: "#44e049" },
        { pattern: /\bWARN\b/g, class: "#c7b53a" },
        { pattern: /\bERROR\b/g, class: "#fa3939" },
        { pattern: /\b\w+(\.\w+)+\b/g, class: "#01fdaf" },
        { pattern: /\[[^\]]*\]: Server stopped/g, class: "red" },
      ];
      let highlightedText = logText;
      regexes.forEach(({ pattern, class: cssClass }) => {
        highlightedText = highlightedText.replace(
          pattern,
          `<span style="color:${cssClass}">$&</span>`
        );
      });
      return highlightedText;
    },
    highlightedJson() {
      const logText = this.editableContent
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;");
      const regexes = [
        { pattern: /".*":/g, class: "#32daf5" },
        { pattern: /.*".*"/g, class: "#65fa5f" },
      ];
      let highlightedText = logText;
      regexes.forEach(({ pattern, class: cssClass }) => {
        highlightedText = highlightedText.replace(
          pattern,
          `<span style="color:${cssClass}">$&</span>`
        );
      });
      return highlightedText;
    },
    highlightedProperties() {
      const logText = this.editableContent
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;");
      const regexes = [
        {
          pattern: /#([A-Za-z0-9]+(( )|()[A-Za-z0-9:]+)+)/gm,
          class: "#999999",
        },
        { pattern: /^([A-Za-z0-9/.+-]+)/gm, class: "#32daf5" },
        {
          pattern: /=([A-Za-z0-9]+(( )|()[A-Za-z0-9+-{}]+)+)/gm,
          class: "#ff7f50",
        },
      ];
      let highlightedText = logText;
      regexes.forEach(({ pattern, class: cssClass }) => {
        highlightedText = highlightedText.replace(
          pattern,
          `<span style="color:${cssClass}">$&</span>`
        );
      });
      return highlightedText;
    },
  },
};
</script>

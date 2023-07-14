<style lang="scss" scoped>
@import "../../assets/scss/global.scss";
$_padding: 10px;

.text-editor {
  background: $fg-a;
  padding: $_padding;
  box-sizing: border-box;
  border-radius: $brr;
  position: relative;

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
      font-size: 1rem;
      padding: 8px;
      * {
        margin: 0;
      }
    }
    .save-button {
      float: right;
      margin-right: 10px;
    }
  }
}

.text-holder {
  display: block;
  width: 100%;
  height: calc(100vh - 120px);
  overflow: auto;
  font-family: "Consolas", monospace;
  white-space: normal;
  box-sizing: border-box;
  margin: 0;
  position: relative;
  overflow: auto;
}

.text,
.write {
  height: 100%;
  border: none;
  padding: 0;
  outline: none;
  resize: none;
  display: block;
  font-size: 1.1rem;
  line-height: 1;
  overflow: auto;
  font-family: Consolas, monospace;
  box-sizing: border-box;
  hyphens: auto;
  white-space: pre;
  overflow-wrap: normal;
  border: none;
}

.write {
  background: transparent;
  color: #8b8b8b56;
  position: absolute;
  top: calc($_padding - 10px);
  border-radius: 0 !important;
  box-shadow: none;
}

.info {
  display: block;
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  width: calc(100% - 8 * $_padding);
  padding: $_padding;
  box-sizing: border-box;
  background-color: var(--color-raised-bg);
  border-radius: var(--radius-md);
  border: 1px solid var(--color-button-bg);

  .buttons {
    display: flex;
    justify-content: space-between;
    margin-top: 10px;
  }
}
</style>

<template>
  <div class="text-editor">
    <h2>
      <Button class="open-button" @click="openInFiles">
        <ExternalIcon />
        <p>Open file</p>
      </Button>
      <SaveButton
        class="save-button"
        ref="savingFile"
        @click="saveFileContent"
        color="highlight"
      />
      <div class="flex-heading">
        <LeftArrowIcon @click="closeTextEditor" />
        <p>{{ fileName }}</p>
      </div>
    </h2>
    <div class="text-holder">
      <div
        class="text"
        ref="textRef"
        v-html="highlight"
        @scroll="syncScroll"
      ></div>
      <textarea
        spellcheck="false"
        class="write"
        ref="textAreaRef"
        v-model="editableContent"
        @scroll="syncScroll"
        :style="{ height: textAreaHeight + 'px', width: textAreaWidth + 'px' }"
      ></textarea>
    </div>
    <div class="info" v-if="info !== ''">
      <h3>{{ info.heading }}</h3>
      {{ info.text }}
      <div class="buttons">
        <Button :action="info.btnAction">{{ info.btnText }}</Button>
        <Button
          class="info-close-btn"
          iconOnly
          :action="
            () => {
              this.info = '';
            }
          "
          ><XIcon
        /></Button>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import { Button, LeftArrowIcon, ExternalIcon, XIcon } from "omorphia";
import SaveButton from "../SaveButton.vue";
export default {
  name: "TextEditor",
  components: {
    Button,
    LeftArrowIcon,
    ExternalIcon,
    SaveButton,
    XIcon,
  },
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
      originalContent: "",
      editableContent: "",
      textAreaHeight: 0,
      textAreaWidth: 0,
      info: "",
    };
  },
  mounted() {
    this.editableContent = this.formatFile(this.fileContent);
    this.originalContent = this.editableContent;
    this.updateTextAreaSize();
    window.addEventListener("resize", this.handleResize);
    this.checkFileName();
  },
  beforeDestroy() {
    window.removeEventListener("resize", this.handleResize);
  },
  watch: {
    editableContent(newContent) {
      this.$emit("updatedContent", newContent);
    },
  },
  emits: ["changeTab", "updatedContent", "CloseTextEditor"],
  methods: {
    saveContentToEdit() {
      this.editableContent = this.$refs.textAreaRef.value;
    },
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
    handleResize() {
      this.updateTextAreaSize();
    },
    checkFileName() {
      switch (this.fileName) {
        case "server.properties": {
          this.openInfo(
            "Enhanced Editor",
            "For an improved editing experience, it's recommend using the 'Properties' tab.",
            "Go to Properties",
            () => {
              this.$emit("changeTab", "properties");
            }
          );
          break;
        }
        case "server-info.json": {
          this.openInfo(
            "Editing Not Recommended",
            "Editing this file can potentially cause crashes or worsen the overall program experience.",
            "Go to Save Editor",
            () => {
              this.$emit("changeTab", "server_info");
            }
          );
          break;
        }
      }
    },
    openInfo(header, text, buttonText, buttonAction) {
      this.info = {
        heading: header,
        text,
        btnText: buttonText,
        btnAction: buttonAction,
      };
    },
    async saveFileContent() {
      if (
        (await invoke("set_server_file", {
          filePath: this.filePath,
          contents: this.editableContent,
          serverId: this.serverId,
        })) == null
      ) {
        this.$refs["savingFile"].success = true;
      } else {
        this.$refs["savingFile"].success = false;
      }
    },
    updateTextAreaSize() {
      const textEl = this.$refs.textRef;
      const textareaEl = this.$refs.textAreaRef;
      if (textEl && textareaEl) {
        this.textAreaHeight = textEl.getBoundingClientRect().height;
        this.textAreaWidth = textEl.getBoundingClientRect().width;
      }
      console.log(this.textAreaHeight, this.textAreaWidth);
    },
    syncScroll(event) {
      const target = event.target;
      const textEl = this.$refs.textRef;
      if (target === textEl) {
        // If the .text element is scrolled, update the textarea's scrollTop
        this.$refs.textAreaRef.scrollTop = textEl.scrollTop;
        this.$refs.textAreaRef.scrollLeft = textEl.scrollLeft;
      } else {
        // If the textarea is scrolled, update the .text element's scrollTop
        textEl.scrollTop = this.$refs.textAreaRef.scrollTop;
        textEl.scrollLeft = this.$refs.textAreaRef.scrollLeft;
      }
    },
    formatFile(content) {
      if (this.fileType === "json") {
        return JSON.stringify(JSON.parse(content), null, 2);
      }
      return content;
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
        default: {
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
      highlightedText = highlightedText
        .split("\n")
        .map((line) => {
          if (line.trim() === "") {
            return line; // Skip empty lines
          } else {
            return `<span class="no-wrap">${line}</span>`;
          }
        })
        .join("\n");
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
      highlightedText = highlightedText
        .split("\n")
        .map((line) => {
          if (line.trim() === "") {
            return line; // Skip empty lines
          } else {
            return `<span class="no-wrap">${line}</span>`;
          }
        })
        .join("\n");
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

      highlightedText = highlightedText
        .split("\n")
        .map((line) => {
          if (line.trim() === "") {
            return line; // Skip empty lines
          } else {
            return `<span class="no-wrap">${line}</span>`;
          }
        })
        .join("\n");
      return highlightedText;
    },
  },
};
</script>

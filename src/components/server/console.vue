<style lang="scss" scoped>
@import "../../assets/scss/global.scss";

.page-in {
  box-sizing: border-box;
}
.console {
  $_height: calc(100vh - 50px);
  max-width: calc(100% - 10px);
  max-height: $_height;
  height: $_height;
  margin: 0 auto;
  overflow: hidden;

  background-color: $fg-m;
  border-radius: $brr;

  .console-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: $fg-ma;
    border-bottom: 1px solid $fg-h;
    margin-bottom: -1px;

    h2 {
      margin: 6px 3px 3px 6px;
    }
    .controls {
      padding-right: 5px;

      .scroll-button {
        display: flex;
        align-items: center;
        font-size: 1.2rem;
        background-color: $fg-h;
        color: $txt-m;
        transition: all 50ms;
        padding: 3px 8px;
        border-radius: 8px;
        border: none;
        cursor: pointer;

        &.active {
          background-color: $main-am;
        }
      }
    }
  }
  .log {
    min-height: calc($_height - 95px);
    max-height: calc($_height - 95px);
    overflow-x: hidden;
    overflow-y: scroll;
    pre {
      padding: 0 10px;
      color: $txt-m;
      white-space: pre-wrap;
      word-break: break-all;
      overflow-wrap: break-word;
    }
  }
  .command-line {
    font-size: 1.1rem;
    display: block;
    margin: 10px;
    width: calc(100% - 20px);
    box-sizing: border-box;
    overflow: hidden;

    color: $txt;
    background-color: $fg-a;
    padding: 0.505rem 0.808rem;
    border: none;
    border-radius: 6px;
    box-sizing: border-box;

    &:disabled {
      background-color: $fg-ma;
      cursor: not-allowed;
    }
  }
}
</style>

<template>
  <div class="page-in">
    <div class="console">
      <div class="console-head">
        <h2>Console</h2>
        <div class="controls">
          <button
            @click="scrollBottom = !scrollBottom"
            class="scroll-button"
            :class="scrollBottom ? 'active' : ''"
          >
            Auto scroll
          </button>
        </div>
      </div>
      <div class="log" ref="logContainer">
        <pre><code v-html="highlightedLog"></code></pre>
      </div>
      <input
        type="text"
        class="command-line"
        @keyup.enter="send_command"
        placeholder="Minecraft command..."
        v-model="serverCommand"
        :disabled="!status"
        :title="!status ? 'Server is offline' : ''"
      />
    </div>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
export default {
  name: "ConsoleView",
  props: {
    serverId: {
      default: "",
      type: String,
    },
    serverInfo: {
      default: {},
      type: Object,
    },
    status: {
      default: false,
      type: Boolean,
    },
  },
  data() {
    return {
      logData: "",
      fetchInterval: null,
      scrollBottom: true,
      serverCommand: "",
    };
  },
  async mounted() {
    this.startFetchingServerLog();
    this.logData = await invoke("get_server_log", {
      serverId: this.serverId,
    });
  },
  beforeUnmount() {
    clearInterval(this.fetchInterval);
  },
  methods: {
    async startFetchingServerLog() {
      this.fetchInterval = setInterval(this.get_server_log, 1000); // Fetch server log every 500ms
    },
    async send_command() {
      let sending = { command: this.serverCommand };
      console.log(await invoke("execute_command_on_server", sending));
      this.serverCommand = "";
    },
    async get_server_log() {
      if (this.status) {
        this.logData = await invoke("get_server_log", {
          serverId: this.serverId,
        });
        this.$nextTick(() => {
          this.scrollLogToBottom();
        });
      }
    },
    scrollLogToBottom() {
      const logContainer = this.$refs.logContainer;
      if (
        this.fetchInterval &&
        this.scrollBottom &&
        logContainer &&
        logContainer.scrollHeight
      ) {
        logContainer.scrollTop = logContainer.scrollHeight;
      }
    },
  },
  computed: {
    highlightedLog() {
      const logText = this.logData.replace(/</g, "&lt;").replace(/>/g, "&gt;");

      // Define the regular expressions and corresponding CSS classes for highlighting
      const regexes = [
        { pattern: /"\b[^"]*\b"/g, class: "#32daf5" },
        { pattern: /\b[0-9]{2}:[0-9]{2}:[0-9]{2}\b/g, class: "#aaa" },
        { pattern: /\bINFO\b/g, class: "#44e049" },
        { pattern: /\bWARN\b/g, class: "#c7b53a" },
        { pattern: /\bERROR\b/g, class: "#fa3939" },
        { pattern: /\b\w+(\.\w+)+\b/g, class: "#01fdaf" },
        { pattern: /\[[^\]]*\]: Server stopped/g, class: "red" },
      ];

      // Apply the highlighting using the regular expressions and CSS classes
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

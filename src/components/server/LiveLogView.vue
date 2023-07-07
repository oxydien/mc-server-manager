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
        font-size: 1.2rem;
        padding: 3px 8px;
      }
    }
  }
  .log {
    min-height: calc($_height - 95px);
    max-height: calc($_height - 95px);
    overflow: auto;
    font-family: "Consolas", monospace;
    white-space: normal;
    padding-bottom: 1.5rem;
    box-sizing: border-box;
  }
  .command-line {
    font-size: 1.1rem;
    margin: 10px;
    width: calc(100% - 20px);
    overflow: hidden;

    &:disabled {
      cursor: not-allowed !important;
    }
  }
}

.not-implemented {
  padding: 1.3rem;
}
</style>

<template>
  <div class="page-in">
    <div class="console">
      <div class="console-head">
        <h2>Live log</h2>
        <div class="controls">
          <Button
            @click="scrollBottom = !scrollBottom"
            class="scroll-button"
            :color="scrollBottom ? 'primary' : ''"
          >
            <DropdownIcon v-show="scrollBottom" />
            Auto scroll
          </Button>
        </div>
      </div>
      <div
        class="log"
        v-if="highlightedLog !== '' || logData === '{}'"
        ref="logContainer"
        v-html="highlightedLog"
      ></div>
      <div class="log" v-else>
        <InfoTable type="server_no_start" />
      </div>
      <input
        type="text"
        class="command-line"
        @keyup.enter="send_command"
        @dblclick="send_command"
        placeholder="Send minecraft command..."
        v-model="serverCommand"
        :disabled="!status"
        :title="!status ? 'Server is offline' : ''"
      />
      <Modal ref="sendCommandModal" header="Feature Not Implemented">
        <div class="not-implemented">
          <p>
            Sorry, sending commands to the server is currently not implemented.
          </p>
          <p>
            If you know how to implement this feature, you can contribute this
            feature to the project.
          </p>
          <p>
            Adding this feature to this software would greatly enhance its
            capabilities.
          </p>
          <blockquote>
            If you have a critical need for this feature, here's a workaround: Once the server starts, it opens a white console window where you can enter your commands.
          </blockquote>
          <Button @click="openLink('https://github.com/oxydien/mc-server-manager/blob/main/src-tauri/src/server/run.rs#L12')"><LinkIcon />Github link</Button>
        </div>
      </Modal>
    </div>
    <Notifications ref="notifsContainer" />
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import { Button, Modal, DropdownIcon, Notifications, LinkIcon } from "omorphia";
import InfoTable from "../icons/InfoTable.vue";
export default {
  name: "ConsoleView",
  components: {
    InfoTable,
    Button,
    Modal,
    DropdownIcon,
    Notifications,
    LinkIcon,
  },
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
      logData: "{}",
      fetchInterval: null,
      scrollBottom: true,
      serverCommand: "",
    };
  },
  async mounted() {
    this.startFetchingServerLog();
    this.askForLog();
  },
  beforeUnmount() {
    clearInterval(this.fetchInterval);
  },
  methods: {
    async startFetchingServerLog() {
      this.fetchInterval = setInterval(this.get_server_log, 1000); // Fetch server log every 1s
    },
    async send_command() {
      // Sending command to the server is not done!
      // please if you know what are you doing (i don't)
      // add this feature to the rust...
      this.$refs.sendCommandModal.show();
      return; // ↓ leaving this code for maybe future usage...
      let sending = { command: this.serverCommand };
      console.log(await invoke("execute_command_on_server", sending));
      this.serverCommand = "";
    },
    openLink(link) {
      window.open(link, '_blank')
    },
    addNotification(title, message, type) {
      this.$refs.notifsContainer.addNotification({
        title,
        text: message,
        type,
      });
    },
    async get_server_log() {
      if (this.status) {
        await askForLog();
        this.$nextTick(() => {
          this.scrollLogToBottom();
        });
      }
    },
    async askForLog() {
      try {
        this.logData = await invoke("get_server_log", {
          serverId: this.serverId,
        });
      } catch (e) {
        this.addNotification("Getting server log", e, "error");
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
      if (this.logData !== "{}" && this.logData !== "") {
        const logText = this.logData
          .replace(/</g, "&lt;")
          .replace(/>/g, "&gt;");

        // Color patterns for log
        const regexes = [
          { pattern: /"\b[^"]*\b"/g, color: "#32daf5" },
          { pattern: /\b[0-9]{2}:[0-9]{2}:[0-9]{2}\b/g, color: "#aaa" },
          { pattern: /\bINFO\b/g, color: "#44e049" },
          { pattern: /\bWARN\b/g, color: "#c7b53a" },
          { pattern: /\bERROR\b/g, color: "#fa3939" },
          { pattern: /\b\w+(\.\w+)+\b/g, color: "#01fdaf" },
          { pattern: /\[[^\]]*\]: Server stopped/g, color: "red" },
        ];

        // Applying colors to log
        let highlightedText = logText;
        regexes.forEach(({ pattern, color: color }) => {
          highlightedText = highlightedText.replace(
            pattern,
            `<span style="color:${color}">$&</span>`
          );
        });

        // Wrap each line with span element
        highlightedText = highlightedText
          .split("\n")
          .map((line) => `<span class="no-wrap">${line}</span>`)
          .join("\n");
        return highlightedText;
      }
      return "";
    },
  },
};
</script>

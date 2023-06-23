<style lang="scss" scoped>
@import "../../assets/scss/global.scss";
.wrapper {
  display: flex;
  gap: 10px;
}

.aside {
  width: 230px;
  min-width: 230px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  overflow-y: auto;
  background-color: $fg-ma;
  border-radius: $brr 2px 2px $brr;
  box-sizing: border-box;
  padding: 5px 5px 0;
  .server-heading {
    background-color: $fg-h;
    padding: 5px;
    box-sizing: border-box;
    border-radius: 10px 10px 5px 5px;
    .image-holder {
      margin: 0 auto;
      width: fit-content;
      svg,
      img {
        width: 200px;
        height: 200px;
        background-color: $fg-ma;
        border-radius: $brr;
        object-fit: cover;
      }
    }
    p,
    h3 {
      margin: 0;
      text-align: center;
      max-width: 100%;
      overflow: hidden;
    }
    p {
      margin-top: 5px;
      text-transform: capitalize;
    }
  }
  .pages {
    display: flex;
    flex-direction: column;
    gap: 5px;
    box-sizing: border-box;

    .page {
      background-color: $fg-a;
      color: $txt-m;
      border: none;
      text-align: left;
      padding: 10px 5px;
      font-size: 1.3rem;
      cursor: pointer;
      border-radius: 5px;

      .page-name {
        display: flex;
        align-content: center;
        gap: 5px;
      }

      &.active-page {
        background-color: $main-am;
      }
      &:hover {
        svg.arrow {
          right: 5px;
        }
      }

      &:active {
        svg.arrow {
          right: -3px;
        }
      }

      svg.arrow {
        position: relative;
        transition: all 50ms ease;
        right: 0px;
        float: right;
      }
    }
  }
}

.content {
  overflow: auto;
  background-color: $fg-ma;
  width: 100%;
  max-width: 100%;
  max-height: 100%;
  border-radius: 2px $brr $brr 2px;
  padding: 5px 5px 0;
  box-sizing: border-box;
}

.buttons {
  display: flex;
  justify-content: center;
  gap: 5px;
  margin-top: 5px;

  .start-button {
    background-color: #1bc73b65;
    border: none;
    font-size: 1.1rem;
    color: $txt-h;
    border-radius: 5px;
    padding: 0.3rem 0.5rem;
    cursor: pointer;

    svg {
      float: left;
      margin-right: 2px;
    }
  }

  .kill-button {
    background-color: #c71b1b65;
    border: none;
    font-size: 1.1rem;
    color: $txt-h;
    border-radius: 5px;
    padding: 0.3rem 0.5rem;
    cursor: pointer;

    svg {
      float: left;
      margin-right: 2px;
    }
  }

  .folder-button {
    background-color: #1b93c765;
    border: none;
    color: $txt-h;
    border-radius: 5px;
    padding: 0.3rem 0.5rem;
    cursor: pointer;

    svg {
      float: left;
    }
  }
}
</style>

<template>
  <main class="wrapper">
    <div class="aside">
      <div class="server-heading">
        <div class="image-holder" :key="serverInfo">
          <img
            :src="serverInfo.image"
            v-if="serverInfo.image"
            @error="replaceImgbySvg"
          />
          <svg
            v-else
            width="512"
            height="512"
            viewBox="0 0 512 512"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M96.7593 163C102.259 145.5 233.259 87 252.259 87C271.259 87 394.259 143 409.759 163M96.7593 163C91.2593 180.5 92.2593 314 96.7593 332.5C101.259 351 224.259 425.5 252.259 425.5M96.7593 163C125.759 198 220.759 244.5 252.259 244.5M409.759 163C422.759 185.5 420.759 313.5 409.759 332.5C398.759 351.5 280.259 425.5 252.259 425.5M409.759 163C378.759 204 286.259 247 252.259 244.5M252.259 425.5C252.259 354.815 252.259 315.185 252.259 244.5"
              stroke="currentColor"
              stroke-width="42"
            />
          </svg>
        </div>
        <h3>{{ serverInfo.name || "loading..." }}</h3>
        <p>
          {{ serverInfo.server_type || "loa" }} -
          {{ serverInfo.mc_version || "ding..." }}
        </p>
        <div class="buttons">
          <button v-if="!status" @click="startServer" class="start-button">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 256 256"
            >
              <path
                fill="currentColor"
                d="M234.49 111.07L90.41 22.94A20 20 0 0 0 60 39.87v176.26a20 20 0 0 0 30.41 16.93l144.08-88.13a19.82 19.82 0 0 0 0-33.86ZM84 208.85V47.15L216.16 128Z"
              />
            </svg>
            Start
          </button>
          <button v-else @click="killServer" class="kill-button">
            <svg
              width="20"
              height="20"
              viewBox="0 0 20 20"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <rect
                x="4"
                y="4"
                width="12"
                height="12"
                rx="2"
                stroke="white"
                stroke-width="2"
              />
            </svg>
            Kill
          </button>
          <button class="folder-button" @click="openInFiles">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
            >
              <path
                fill="currentColor"
                d="M6.1 10L4 18V8h17a2 2 0 0 0-2-2h-7l-2-2H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h15c.9 0 1.7-.6 1.9-1.5l2.3-8.5H6.1M19 18H6l1.6-6h13L19 18Z"
              />
            </svg>
          </button>
        </div>
      </div>
      <div class="pages">
        <button
          @click="goToPage(0)"
          :class="(page == 0 ? 'active-page' : '') + ' page'"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            class="arrow"
            viewBox="0 0 24 24"
          >
            <path
              fill="currentColor"
              d="M8.59 16.58L13.17 12L8.59 7.41L10 6l6 6l-6 6l-1.41-1.42Z"
            />
          </svg>
          <div class="page-name">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="22"
              height="22"
              viewBox="0 0 20 20"
            >
              <path
                fill="currentColor"
                d="M5.646 9.146a.5.5 0 0 1 .708 0l2 2a.5.5 0 0 1 0 .708l-2 2a.5.5 0 0 1-.708-.708L7.293 11.5L5.646 9.854a.5.5 0 0 1 0-.708ZM14 13H9.5a.5.5 0 0 0 0 1H14a.5.5 0 0 0 0-1ZM3 5.5A2.5 2.5 0 0 1 5.5 3h9A2.5 2.5 0 0 1 17 5.5v9a2.5 2.5 0 0 1-2.5 2.5h-9A2.5 2.5 0 0 1 3 14.5v-9ZM4.5 7v7.25c0 .69.56 1.25 1.25 1.25h8.5c.69 0 1.25-.56 1.25-1.25V7h-11Z"
              />
            </svg>
            Console
          </div>
        </button>
        <button
          @click="goToPage(1)"
          :class="(page == 1 ? 'active-page' : '') + ' page'"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            class="arrow"
            viewBox="0 0 24 24"
          >
            <path
              fill="currentColor"
              d="M8.59 16.58L13.17 12L8.59 7.41L10 6l6 6l-6 6l-1.41-1.42Z"
            />
          </svg>
          <div class="page-name">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="22"
              height="22"
              viewBox="0 0 24 24"
            >
              <g fill="currentColor">
                <path
                  d="M11 10.98a1 1 0 1 1 2 0v6a1 1 0 1 1-2 0v-6Zm1-4.929a1 1 0 1 0 0 2a1 1 0 0 0 0-2Z"
                />
                <path
                  fill-rule="evenodd"
                  d="M12 2C6.477 2 2 6.477 2 12s4.477 10 10 10s10-4.477 10-10S17.523 2 12 2ZM4 12a8 8 0 1 0 16 0a8 8 0 0 0-16 0Z"
                  clip-rule="evenodd"
                />
              </g>
            </svg>
            Server info
          </div>
        </button>
        <button
          @click="goToPage(2)"
          :class="(page == 2 ? 'active-page' : '') + ' page'"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            class="arrow"
            viewBox="0 0 24 24"
          >
            <path
              fill="currentColor"
              d="M8.59 16.58L13.17 12L8.59 7.41L10 6l6 6l-6 6l-1.41-1.42Z"
            />
          </svg>
          <div class="page-name">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="22"
              height="22"
              viewBox="0 0 24 24"
            >
              <g id="evaOptions2Outline0">
                <g id="evaOptions2Outline1">
                  <path
                    id="evaOptions2Outline2"
                    fill="currentColor"
                    d="M19 9a3 3 0 0 0-2.82 2H3a1 1 0 0 0 0 2h13.18A3 3 0 1 0 19 9Zm0 4a1 1 0 1 1 1-1a1 1 0 0 1-1 1ZM3 7h1.18a3 3 0 0 0 5.64 0H21a1 1 0 0 0 0-2H9.82a3 3 0 0 0-5.64 0H3a1 1 0 0 0 0 2Zm4-2a1 1 0 1 1-1 1a1 1 0 0 1 1-1Zm14 12h-7.18a3 3 0 0 0-5.64 0H3a1 1 0 0 0 0 2h5.18a3 3 0 0 0 5.64 0H21a1 1 0 0 0 0-2Zm-10 2a1 1 0 1 1 1-1a1 1 0 0 1-1 1Z"
                  />
                </g>
              </g>
            </svg>
            Properties
          </div>
        </button>
        <button
          @click="goToPage(3)"
          :class="(page == 3 ? 'active-page' : '') + ' page'"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            class="arrow"
            viewBox="0 0 24 24"
          >
            <path
              fill="currentColor"
              d="M8.59 16.58L13.17 12L8.59 7.41L10 6l6 6l-6 6l-1.41-1.42Z"
            />
          </svg>
          <div class="page-name">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="22"
              height="22  "
              viewBox="0 0 24 24"
            >
              <path
                fill="currentColor"
                d="M20 9.502V8.75a2.25 2.25 0 0 0-2.25-2.25h-5.725l-2.38-1.98A2.25 2.25 0 0 0 8.204 4H4.25A2.25 2.25 0 0 0 2 6.25l-.004 11.5A2.25 2.25 0 0 0 4.246 20H18.47a1.75 1.75 0 0 0 1.698-1.325l1.75-6.998a1.75 1.75 0 0 0-1.698-2.175H20ZM4.25 5.5h3.956a.75.75 0 0 1 .48.173l2.588 2.154a.75.75 0 0 0 .48.173h5.996a.75.75 0 0 1 .75.75v.752H6.424a2.25 2.25 0 0 0-2.183 1.704l-.744 2.978L3.5 6.25a.75.75 0 0 1 .75-.75Zm1.447 6.07a.75.75 0 0 1 .727-.568H20.22a.25.25 0 0 1 .242.31l-1.75 6.999a.25.25 0 0 1-.242.189H4.285a.25.25 0 0 1-.243-.31l1.655-6.62Z"
              />
            </svg>
            Files
          </div>
        </button>
        <button
          @click="goToPage(9)"
          v-if="
            serverInfo.server_type == 'quilt' ||
            serverInfo.server_type == 'fabric'
          "
          :class="(page == 9 ? 'active-page' : '') + ' page'"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            class="arrow"
            viewBox="0 0 24 24"
          >
            <path
              fill="currentColor"
              d="M8.59 16.58L13.17 12L8.59 7.41L10 6l6 6l-6 6l-1.41-1.42Z"
            />
          </svg>
          <div class="page-name">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="22"
              height="22"
              viewBox="0 0 32 32"
            >
              <path
                fill="currentColor"
                d="m27.84 6.775l-3.198 3.24l-2.872-.77L21 6.373l3.187-3.23a5.727 5.727 0 0 0-5.848 1.39a5.738 5.738 0 0 0-1.28 6.172l-9.64 9.64a3.971 3.971 0 0 0-.62-.062a3.932 3.932 0 0 0-3.934 3.933a3.932 3.932 0 1 0 7.865 0c0-.24-.03-.473-.07-.7l9.59-9.59a5.75 5.75 0 0 0 6.203-1.272a5.733 5.733 0 0 0 1.384-5.878zM6.8 25.145a.934.934 0 0 1-.936-.932a.932.932 0 1 1 .935.933z"
              />
            </svg>
            Mods
          </div>
        </button>
      </div>
    </div>
    <div class="content">
      <div v-show="page == 0">
        <ConsoleView
          :serverId="serverid"
          :serverInfo="serverInfo"
          :status="status"
          :key="serverid + serverInfo"
        ></ConsoleView>
      </div>
      <div v-show="page == 1">
        <ServerInfo
          :server_id="serverid"
          :status="status"
          :key="serverid"
        ></ServerInfo>
      </div>
      <div v-show="page == 2">
        <PropertiesEditor
          :server_id="serverid"
          :key="serverid"
        ></PropertiesEditor>
      </div>
      <div v-show="page == 3">
        <FileSystem :serverId="serverid" :key="serverid"></FileSystem>
      </div>
      <div
        v-show="page == 9"
        v-if="
          serverInfo.server_type == 'quilt' ||
          serverInfo.server_type == 'fabric'
        "
      >
        <ModrinthMods
          :serverId="serverid"
          :key="serverid + serverInfo.server_type + serverInfo.mc_version"
          :serverType="serverInfo.server_type"
          :serverVersion="serverInfo.mc_version"
        ></ModrinthMods>
      </div>
    </div>
  </main>
</template>

<script>
import ServerInfo from "./serverInfo.vue";
import ConsoleView from "./console.vue";
import FileSystem from "./files.vue";
import PropertiesEditor from "./propertiesEditor.vue";
import ModrinthMods from "./downloadMods.vue";
import { invoke } from "@tauri-apps/api/tauri";
export default {
  name: "ServerView",
  data() {
    return {
      page: 0,
      serverid: "",
      serverInfo: {},
      status: false,
      serverStatusInterval: {},
    };
  },
  components: {
    ServerInfo,
    ConsoleView,
    FileSystem,
    PropertiesEditor,
    ModrinthMods,
  },
  mounted() {
    this.getServerInfo();
    this.startServerStatusPolling();
  },
  beforeUnmount() {
    clearInterval(this.serverStatusInterval);
  },
  methods: {
    startServerStatusPolling() {
      this.serverStatusInterval = setInterval(() => {
        this.getServerStatus();
      }, 500);
    },
    async openInFiles() {
      console.log(
        await invoke("open_file_or_explorer", {
          serverId: this.serverid,
          path: "./",
        })
      );
    },
    async startServer() {
      console.log(
        await invoke("start_server_command", {
          serverId: this.serverid,
          serverType: this.serverInfo.server_type,
        })
      );
    },
    async killServer() {
      console.log(
        await invoke("kill_server_command", {
          serverId: this.serverid,
        })
      );
    },
    async getServerStatus() {
      const response = await invoke("get_server_status_command", {
        serverId: this.serverid,
      });
      this.status = response;
    },
    goToPage(page) {
      this.page = page;
    },
    async getServerInfo() {
      this.serverid = this.$route.params.id;
      if (this.serverid != "") {
        this.serverInfo = JSON.parse(
          await invoke("get_server_info_command", { serverId: this.serverid })
        );
      }
    },
    replaceImgbySvg() {
      this.serverInfo.image = null;
    },
  },
};
</script>

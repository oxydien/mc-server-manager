<style lang="scss" scoped>
@import "../../assets/scss/global.scss";
.wrapper {
  display: flex;
  gap: 10px;
}

.aside {
  display: flex;
  flex-direction: column;
  gap: 10px;

  width: 230px;
  min-width: 230px;

  background-color: var(--color-bg);
  
  overflow-y: auto;
  border-radius: $brr 2px 2px $brr;
  box-sizing: border-box;
  padding: 5px 5px 0;
  .server-heading {
    background-color: var(--color-raised-bg);
    padding: 15px 5px;
    box-sizing: border-box;
    border-radius: var(--radius-md) var(--radius-md) var(--radius-sm) var(--radius-sm);
    text-align: center;
    p,
    h3 {
      margin: 0;
      max-width: 100%;
      overflow: hidden;
    }
    p {
      margin-top: 5px;
      text-transform: capitalize;
      svg {
        height: 12px;
        width: 12px;
      }
    }
  }
  .pages {
    display: flex;
    flex-direction: column;
    gap: 5px;

    .page {
      font-size: 1.3rem;
      width: 100%;
    }
  }
}

.content {
  overflow: auto;
  background-color: var(--color-bg);
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
    background-color: #12a539b3;
    font-size: 1.1rem;
    height: 2.25rem;
  }

  .kill-button {
    background-color: var(--color-red);
    font-size: 1.1rem;
    height: 2.25rem;
  }

  .folder-button {
    background-color: var(--color-ad-raised);
  }
}
</style>

<template>
  <main class="wrapper">
    <div class="aside">
      <div class="server-heading">
        <Avatar size="lg" :src="serverInfo.image" />
        <h3>{{ serverInfo.name || "loading..." }}</h3>
        <p>
          {{ serverInfo.s_type || "loa" }}
          <ServerTypeIcon :type="serverInfo.s_type || 'null'" /> -
          {{ serverInfo.mc_version || "ding..." }}
        </p>
        <div class="buttons">
          <Button v-if="!status" @click="startServer" class="start-button">
            <PlayIcon />
            Start
          </Button>
          <Button v-else @click="killServer" class="kill-button">
            <StopCircleIcon />
            Kill
          </Button>
          <Button iconOnly class="folder-button" @click="openInFiles">
            <FolderOpenIcon />
          </Button>
        </div>
      </div>
      <div class="pages">
        <Button
          @click="goToPage(0)"
          class="page"
          :color="page == 0 ? 'primary' : ''"
        >
          <FileIcon />
          Live log
        </Button>
        <Button
          @click="goToPage(1)"
          class="page"
          :color="page == 1 ? 'primary' : ''"
        >
          <InfoIcon />
          Server info
        </Button>
        <Button
          @click="goToPage(2)"
          class="page"
          :color="page == 2 ? 'primary' : ''"
        >
          <SettingsIcon />
          Properties
        </Button>
        <Button
          @click="goToPage(3)"
          class="page"
          :color="page == 3 ? 'primary' : ''"
        >
          <FolderOpenIcon />
          Files
        </Button>
        <Button
          @click="goToPage(9)"
          v-if="
            serverInfo.s_type == 'quilt' ||
            serverInfo.s_type == 'fabric'
          "
          class="page"
          :color="page == 9 ? 'primary' : ''"
        >
          <ServerTypeIcon :type="serverInfo.s_type" />
          Mods
        </Button>
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
          serverInfo.s_type == 'quilt' ||
          serverInfo.s_type == 'fabric'
        "
      >
        <!-- <ModrinthMods
          :serverId="serverid"
          :key="serverid + serverInfo.s_type + serverInfo.mc_version"
          :serverType="serverInfo.s_type"
          :serverVersion="serverInfo.mc_version"
        ></ModrinthMods> -->
      </div>
    </div>
  </main>
</template>

<script>
import ServerInfo from "./serverInfo.vue";
import ConsoleView from "./LiveLogView.vue";
import FileSystem from "./files.vue";
import PropertiesEditor from "./propertiesEditor.vue";
// import ModrinthMods from "./downloadMods.vue";
import ServerTypeIcon from "../icons/ServerTypeIcon.vue";
import { invoke } from "@tauri-apps/api/tauri";
import {
  Avatar,
  PlayIcon,
  Button,
  StopCircleIcon,
  FolderOpenIcon,
  InfoIcon,
  FileIcon,
  SettingsIcon,
} from "omorphia";
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
    // ModrinthMods,
    Avatar,
    ServerTypeIcon,
    PlayIcon,
    Button,
    StopCircleIcon,
    FolderOpenIcon,
    InfoIcon,
    FileIcon,
    SettingsIcon,
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
          serverType: this.serverInfo.s_type,
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

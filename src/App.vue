<style scoped lang="scss">
@import "./assets/scss/global.scss";

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
.nothing {
  display: none;
  visibility: none;
  height: 0;
  width: 0;
}
</style>

<template>
  <div class="container">
    <nav class="server-list">
      <nav :key="servers.length">
        <button
          @click="openServer(server.id)"
          :data-server-name="server.name"
          v-for="(server, index) in servers"
          :key="index"
          class="side-server"
          :class="$route.path === `/server/${server.id}`?'open':''"
        >
          <Avatar size="sm" :src="server.image" />
        </button>
        <hr v-if="servers.length > 0" style="width: 80%" />
        <a
          @click="createNewServer"
          data-server-name="Create new server"
          class="side-server create-new"
          :class="$route.path === `/add/server/`?'open':''"
          >
          <PlusIcon></PlusIcon>
        </a>
      </nav>
      <nav>
        <a @click="goHome" class="home-button" :class="$route.path === `/`?'open':''">
          <homeVue></homeVue>
        </a>
        <a @click="goToSettings" class="home-button" :class="$route.path === `/settings`?'open':''">
          <SettingsIcon> </SettingsIcon>
        </a>
      </nav>
    </nav>
    <router-view ref="mainView"> </router-view>
  </div>
  <input class="nothing" ref="nothing" />
</template>

<script>
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Avatar, Button, PlusIcon, SettingsIcon } from "omorphia";
import homeVue from "./components/icons/home.vue";

export default defineComponent({
  name: "App",
  data() {
    return {
      servers: [],
      images: [],
    };
  },
  components: {
    Avatar,
    Button,
    PlusIcon,
    SettingsIcon,
    homeVue
  },
  async mounted() {
    this.loadServers();
    setInterval(() => {
      this.loadServers();
    }, 5000);
  },
  methods: {
    replaceWithSVG(index) {
      console.log(`Image at index ${index} failed to load.`);
      this.servers[index].image = null;
    },
    async getAppdataLoc() {
      const response = await invoke("get_file_loc_command");
      return response;
    },
    async loadServers() {
      this.servers = JSON.parse(await invoke("list_servers"));
    },
    createNewServer() {
      this.$router.push("/add/server/");
    },
    openServer(serverId) {
      this.$router.push(`/`).then(() => {
        this.$router.push(`/server/${serverId}`);
        this.$refs["nothing"].focus();
      });
    },
    goHome() {
      this.$router.push(`/`);
    },
    goToSettings() {
      this.$router.push(`/settings`);
    },
  },
});
</script>

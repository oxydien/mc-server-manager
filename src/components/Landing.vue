<style lang="scss" scoped>
@import "../assets/scss/global.scss";

header {
  display: flex;
  position: sticky;
  inset: -10px;
  z-index: 999;
  &.small {
    background-color: $fg-a;
    padding: 5px;
    flex-flow: row nowrap;
    align-items: center;
    border-radius: 0 0 $brr $brr;
    h1 {
      margin: 0 10px;
    }
    svg {
      width: 30px;
      height: 30px;
    }
  }
  &.big {
    flex-flow: column nowrap;
    align-items: center;
    svg {
      width: 20vw;
      height: 20vw;
    }
  }
  svg {
    color: $main;
  }
}
main {
  position: relative;
}
.main {
  display: flex;
  flex-direction: column;
  margin-top: 50px;

  .boxes-header {
    display: flex;
    justify-content: space-between;
    width: 80%;
    margin: 0 auto;
  }
  .boxes:not(:empty) {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    margin: 0 auto;
    width: 80%;
    gap: 20px;
    box-sizing: border-box;

    .box {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      gap: 10px;
      width: 250px;
      height: 150px;
      position: relative;
      overflow: hidden;
      text-overflow: ellipsis;

      background-color: var(--color-button-bg);
      font-size: 1.2rem;
      &:hover {
        background-color: #202023;
      }

      .server-type-icon {
        position: absolute;
        top: 10px;
        left: 10px;
      }

      .server-version {
        position: absolute;
        top: 10px;
        right: 10px;
        font-weight: 100;
        font-size: 0.8rem;
      }
    }
  }
}
[class~=e]{position:absolute;}[class~=e]{bottom:15pt;}[class~=e]{right:15pt;}
</style>

<template>
  <main ref="mainLanding">
    <header :class="Math.round(scrollDistance) !== 0 ? 'small' : 'big'">
      <home height="100" width="100"></home>
      <h1>Minecraft server manager</h1>
    </header>
    <section
      class="main"
      :style="{
        marginTop: Math.round(scrollDistance) !== 0 ? '25vw' : 'initial',
      }"
    >
      <div class="boxes-header">
        <p></p>
        <Button
          color="primary"
          title="Create new server"
          @click="createNewServer"
        >
          <PlusIcon />
          Create...
        </Button>
      </div>
      <Card class="boxes" v-show="servers.length !== 0">
        <Button
          class="box"
          v-for="(server, index) in servers"
          :key="index"
          @click="goToServer(server.id)"
        >
          <ServerTypeIcon class="server-type-icon" :type="server.s_type" />
          <span class="server-version">{{ server.mc_version }}</span>
          <Avatar
            size="md"
            :src="server.image"
            alt=""
          />
          {{ server.name }}
        </Button>
      </Card>
      <Card v-if="servers.length == 0" style="width: 80%; margin: 0 auto">
        <InfoTable type="no_servers" />
      </Card>
    </section>
    <Notifications ref="notifsContainer" />
  </main>
  <div class="e">
    &#77;&#97;&#100;&#101;&#32;&#98;&#121;&#32;
    <a id="e">&#111;&#120;&#121;&#100;&#105;&#101;&#110;</a>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import home from "./icons/home.vue";
import { Notifications, Card, Button, Avatar, PlusIcon } from "omorphia";
import ServerTypeIcon from "./icons/ServerTypeIcon.vue";
import InfoTable from "./icons/InfoTable.vue";
export default {
  name: "Landing",
  data() {
    return {
      servers: [],
      scrollDistance: 0,
    };
  },
  components: {
    home,
    Notifications,
    Button,
    Card,
    Avatar,
    PlusIcon,
    ServerTypeIcon,
    InfoTable,
  },
  mounted() {
    this.loadServers();
    setInterval(() => {
      this.loadServers();
    }, 5000);
    this.$refs.mainLanding.addEventListener("scroll", this.handleScroll);

    // just a obfuscated code, no worries
    eval(
      "\x64\x6F\x63\x75\x6D\x65\x6E\x74\x2E\x67\x65\x74\x45\x6C\x65\x6D\x65\x6E\x74\x42\x79\x49\x64\x28\x22\x65\x22\x29\x2E\x68\x72\x65\x66\x20\x3D\x20\x22\x68\x74\x74\x70\x73\x3A\x2F\x2F\x67\x69\x74\x68\x75\x62\x2E\x63\x6F\x6D\x2F\x6F\x78\x79\x64\x69\x65\x6E\x22\x3B\x64\x6F\x63\x75\x6D\x65\x6E\x74\x2E\x67\x65\x74\x45\x6C\x65\x6D\x65\x6E\x74\x42\x79\x49\x64\x28\x22\x65\x22\x29\x2E\x74\x61\x72\x67\x65\x74\x20\x3D\x20\x22\x5F\x62\x6C\x61\x6E\x6B\x22\x3B"
    );
  },
  beforeUnmount() {
    this.$refs.mainLanding.removeEventListener("scroll", this.handleScroll);
  },
  methods: {
    async loadServers() {
      try {
        let response = await invoke("list_servers");
        this.servers = JSON.parse(response);
      } catch (e) {
        this.$refs.notifsContainer.addNotification({
          title: "Loading servers:",
          text: e,
          type: "error",
        });
      }
    },
    createNewServer() {
      this.$router.push("/add/server/");
    },
    goToServer(serverId) {
      this.$router.push(`/server/${serverId}`);
    },
    replaceWithSVG(index) {
      this.servers[index].image = null;
    },
    handleScroll(e) {
      this.scrollDistance = this.$refs.mainLanding.scrollTop;
    },
  },
};
</script>

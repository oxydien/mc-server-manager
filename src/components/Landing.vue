<style lang="scss" scoped>
@import "../assets/scss/global.scss";
h1 {
  font-size: clamp(1rem, 5vw, 6rem);
  font-family: "Lucida Sans", "Lucida Sans Regular", "Lucida Grande",
    "Lucida Sans Unicode", Geneva, Verdana, sans-serif;
  text-align: center;
  text-transform: uppercase;
}
.main {
  display: flex;
  flex-direction: column;

  .boxes-header {
    display: flex;
    justify-content: space-between;
    width: 80%;
    margin: 0 auto;

    .new-server {
      display: flex;
      align-items: center;
      font-size: 1.2rem;
      background-color: $fg-h;
      color: $txt-m;
      transition: all 150ms;
      padding: 5px 11px;
      border-radius: 8px;
      border: none;
      cursor: pointer;
      &:hover {
        background-color: $fg-ma;
        color: $main-h;
      }
    }
  }
  .boxes:not(:empty) {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    background-color: $fg-ma;
    padding: 10px;
    border-radius: $brr;
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

      background-color: #2c2c31;
      color: $txt-m;
      transition: all 150ms;
      border-radius: 15px;
      border: none;
      cursor: pointer;
      font-size: 1.2rem;
      &:hover {
        background-color: #202023;
      }

      .server-icon {
        width: 80px;
        height: 80px;
        background-color: $fg-a;
        border-radius: 15px;

        img {
          display: block;
          margin: 0 auto;
          object-fit: cover;
          height: 100%;
          width: 100%;
          border-radius: 10px;
        }
      }
    }
  }
}
</style>

<template>
  <main>
    <h1>
      Minecraft<br />
      Server Manager
    </h1>
    <section class="main">
      <div class="boxes-header">
        <p></p>
        <button class="new-server" title="Create new server" @click="createNewServer">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="32"
              height="30"
              viewBox="0 0 24 24"
            >
              <path
                fill="currentColor"
                d="M13 19.3v-6.7l6-3.4V13c.7 0 1.4.1 2 .4V7.5c0-.4-.2-.7-.5-.9l-7.9-4.4c-.2-.1-.4-.2-.6-.2s-.4.1-.6.2L3.5 6.6c-.3.2-.5.5-.5.9v9c0 .4.2.7.5.9l7.9 4.4c.2.1.4.2.6.2s.4-.1.6-.2l.9-.5c-.3-.6-.4-1.3-.5-2M12 4.2l6 3.3l-2 1.1l-5.9-3.4l1.9-1m-1 15.1l-6-3.4V9.2l6 3.4v6.7m1-8.5L6 7.5l2-1.2l6 3.5l-2 1m8 4.2v3h3v2h-3v3h-2v-3h-3v-2h3v-3h2Z"
              />
            </svg>
          Create...
        </button>
      </div>
      <div class="boxes">
        <button
          class="box"
          v-for="(server, index) in servers"
          :key="index"
          @click="goToServer(server.id)"
        >
          <div class="server-icon">
            <img
              :src="server.image"
              v-if="server.image"
              @error="replaceWithSVG(index)"
              alt=""
            />
            <svg
              v-else
              width="512"
              height="512"
              viewBox="0 0 512 512"
              style="width: 100%; height: 100%"
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
          {{ server.name }}
        </button>
      </div>
    </section>
  </main>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
export default {
  name: "Landing",
  data() {
    return {
      servers: [],
    };
  },
  mounted() {
    this.loadServers();
    setInterval(() => {
      this.loadServers();
    }, 5000);
  },
  methods: {
    async loadServers() {
      this.servers = JSON.parse(await invoke("list_servers"));
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
  },
};
</script>

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
          <img
            :ref="`image-${index}`"
            v-if="server.image"
            :src="server.image"
            @error="replaceWithSVG(index)"
          />
          <svg
            width="512"
            height="512"
            viewBox="0 0 512 512"
            style="width: 100%; height: 100%"
            fill="none"
            v-else
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M96.7593 163C102.259 145.5 233.259 87 252.259 87C271.259 87 394.259 143 409.759 163M96.7593 163C91.2593 180.5 92.2593 314 96.7593 332.5C101.259 351 224.259 425.5 252.259 425.5M96.7593 163C125.759 198 220.759 244.5 252.259 244.5M409.759 163C422.759 185.5 420.759 313.5 409.759 332.5C398.759 351.5 280.259 425.5 252.259 425.5M409.759 163C378.759 204 286.259 247 252.259 244.5M252.259 425.5C252.259 354.815 252.259 315.185 252.259 244.5"
              stroke="currentColor"
              stroke-width="42"
            />
          </svg>
        </button>
        <hr v-if="servers.length > 0" style="width: 80%" />
        <a
          @click="createNewServer"
          data-server-name="Create new server"
          class="side-server create-new"
          :class="$route.path === `/add/server/`?'open':''"
          ><svg
            style="width: 100%; height: 100%"
            xmlns="http://www.w3.org/2000/svg"
            width="30"
            height="30"
            viewBox="0 0 24 24"
          >
            <path fill="currentColor" d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2Z" />
          </svg>
        </a>
      </nav>
      <nav>
        <a @click="goHome" class="home-button" :class="$route.path === `/`?'open':''">
          <svg
            width="40"
            height="40"
            viewBox="0 0 512 512"
            fill="none"
            style="margin: 4px"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M398.85 181.61L430 163.987C418.456 138.056 401.767 114.911 381.226 95.8442L357.572 122.866C375.146 139.232 389.301 159.208 398.85 181.61Z"
              fill="currentColor"
            />
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M209.057 455L213.824 419.978C174.844 414.182 140.109 395.412 114.17 368.21L90.0196 394.85C121.323 426.594 162.739 448.377 209.057 455Z"
              fill="currentColor"
            />
            <path
              d="M238.953 135.42C270.816 135.42 299.625 148.469 320.307 169.506L342.202 146.521C321.307 126.64 304.631 115.335 281.816 109.317C274.017 107.26 265.501 105.821 255.853 104.868C250.58 104.347 244.969 103.972 238.953 103.72L238.953 135.42Z"
              fill="currentColor"
            />
            <path
              d="M30 248.578C30 305.563 52.9013 357.209 90.0196 394.85L114.17 368.21C92.5374 342.991 79.5811 323.618 72.7425 299.515L67.0707 300.928L60.9488 276.451L67.9866 274.697C67.939 274.278 67.8925 273.856 67.8472 273.433C67.0491 265.975 66.6131 257.95 66.4931 249.165C66.4931 199.311 87.6585 154.391 121.51 122.866C152.426 94.0764 193.333 76.722 238.953 76.722C241.038 76.7976 243.293 76.8509 245.618 76.9059C251.09 77.0354 256.951 77.1741 261.908 77.6336C269.56 78.343 279.105 80.4627 287.218 82.3331C312.528 89.3824 332.448 100.793 357.572 122.866L381.226 95.8442C343.922 61.2169 293.916 40 238.953 40C183.99 40 133.983 61.2169 96.6796 95.8442C55.6598 133.921 30 188.259 30 248.578Z"
              fill="currentColor"
            />
            <path
              d="M99.3098 291.894C97.4319 285.435 96.0241 278.568 95.015 271.032C94.8838 270.052 94.7593 269.06 94.6414 268.057L67.9866 274.697C68.9425 283.117 70.3715 290.815 72.3417 298.071C72.4729 298.554 72.6065 299.036 72.7425 299.515L99.5852 292.828C99.4923 292.518 99.4005 292.206 99.3098 291.894Z"
              fill="currentColor"
            />
            <path
              d="M60.9488 276.451L67.0707 300.928L72.7425 299.515C72.6065 299.036 72.4729 298.554 72.3417 298.071C70.3715 290.815 68.9425 283.117 67.9866 274.697L60.9488 276.451Z"
              fill="currentColor"
            />
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M124.982 249.165C124.982 280.185 137.424 308.305 157.598 328.825L135.703 351.81C117.056 330.342 105.828 313.665 99.5852 292.828L107.947 290.745L101.825 266.268L94.6414 268.057C93.9536 262.203 93.49 255.944 93.2197 249.165C93.2197 209.104 109.449 172.825 135.703 146.521L157.598 169.506C137.424 190.026 124.982 218.146 124.982 249.165Z"
              fill="currentColor"
            />
            <path
              d="M95.015 271.032C96.0241 278.568 97.4319 285.435 99.3098 291.894C99.4005 292.206 99.4923 292.518 99.5852 292.828L107.947 290.745L101.825 266.268L94.6414 268.057C94.7593 269.06 94.8838 270.052 95.015 271.032Z"
              fill="currentColor"
            />
            <path
              d="M244.25 110.095L246.498 69.4095L271.865 70.8057L269.617 111.491L244.25 110.095Z"
              fill="currentColor"
            />
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M165.827 298.333L186.724 283.58C179.691 273.311 175.576 260.886 175.576 247.5C175.576 212.196 204.196 183.576 239.5 183.576C259.074 183.576 276.593 192.374 288.319 206.23L319.305 206.944C304.514 177.896 274.331 158 239.5 158C190.071 158 150 198.071 150 247.5C150 266.38 155.846 283.895 165.827 298.333Z"
              fill="currentColor"
            />
            <path
              d="M242.128 311.371C241.257 311.406 240.38 311.424 239.5 311.424C224.924 311.424 211.488 306.546 200.735 298.333L179.878 314.25C195.707 328.399 216.599 337 239.5 337C240.379 337 241.255 336.987 242.128 336.962V311.371Z"
              fill="currentColor"
            />
            <ellipse
              cx="285.5"
              cy="247"
              rx="10.5"
              ry="11"
              fill="currentColor"
            />
            <ellipse
              cx="285.5"
              cy="337"
              rx="10.5"
              ry="11"
              fill="currentColor"
            />
            <ellipse
              cx="285.5"
              cy="427"
              rx="10.5"
              ry="11"
              fill="currentColor"
            />
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M482 217C482 208.716 475.284 202 467 202H255C246.716 202 240 208.716 240 217V278C240 286.284 246.716 293 255 293H467C475.284 293 482 286.284 482 278V217ZM459.165 238.049C459.165 229.765 452.449 223.049 444.165 223.049H277.835C269.551 223.049 262.835 229.765 262.835 238.049V256.951C262.835 265.235 269.551 271.951 277.835 271.951H444.165C452.449 271.951 459.165 265.236 459.165 256.951V238.049Z"
              fill="currentColor"
            />
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M482 307C482 298.716 475.284 292 467 292H255C246.716 292 240 298.716 240 307V367C240 375.284 246.716 382 255 382H467C475.284 382 482 375.284 482 367V307ZM459.165 327.818C459.165 319.533 452.449 312.818 444.165 312.818H277.835C269.551 312.818 262.835 319.533 262.835 327.818V346.183C262.835 354.467 269.551 361.183 277.835 361.183H444.165C452.449 361.183 459.165 354.467 459.165 346.183V327.818Z"
              fill="currentColor"
            />
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M482 396C482 387.716 475.284 381 467 381H255C246.716 381 240 387.716 240 396V457C240 465.284 246.716 472 255 472H467C475.284 472 482 465.284 482 457V396ZM459.165 417.049C459.165 408.765 452.449 402.049 444.165 402.049H277.835C269.551 402.049 262.835 408.765 262.835 417.049V435.951C262.835 444.235 269.551 450.951 277.835 450.951H444.165C452.449 450.951 459.165 444.236 459.165 435.951V417.049Z"
              fill="currentColor"
            />
          </svg>
        </a>
        <a @click="goToSettings" class="home-button" :class="$route.path === `/settings`?'open':''">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="30"
            height="30"
            style="margin: 8px"
            viewBox="0 0 512 512"
          >
            <path
              fill="currentColor"
              d="M256 176a80 80 0 1 0 80 80a80.24 80.24 0 0 0-80-80Zm172.72 80a165.53 165.53 0 0 1-1.64 22.34l48.69 38.12a11.59 11.59 0 0 1 2.63 14.78l-46.06 79.52a11.64 11.64 0 0 1-14.14 4.93l-57.25-23a176.56 176.56 0 0 1-38.82 22.67l-8.56 60.78a11.93 11.93 0 0 1-11.51 9.86h-92.12a12 12 0 0 1-11.51-9.53l-8.56-60.78A169.3 169.3 0 0 1 151.05 393L93.8 416a11.64 11.64 0 0 1-14.14-4.92L33.6 331.57a11.59 11.59 0 0 1 2.63-14.78l48.69-38.12A174.58 174.58 0 0 1 83.28 256a165.53 165.53 0 0 1 1.64-22.34l-48.69-38.12a11.59 11.59 0 0 1-2.63-14.78l46.06-79.52a11.64 11.64 0 0 1 14.14-4.93l57.25 23a176.56 176.56 0 0 1 38.82-22.67l8.56-60.78A11.93 11.93 0 0 1 209.94 26h92.12a12 12 0 0 1 11.51 9.53l8.56 60.78A169.3 169.3 0 0 1 361 119l57.2-23a11.64 11.64 0 0 1 14.14 4.92l46.06 79.52a11.59 11.59 0 0 1-2.63 14.78l-48.69 38.12a174.58 174.58 0 0 1 1.64 22.66Z"
            />
          </svg>
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

export default defineComponent({
  name: "App",
  data() {
    return {
      servers: [],
      images: [],
    };
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

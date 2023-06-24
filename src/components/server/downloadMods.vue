<style lang="scss" scoped>
@import "../../assets/scss/global.scss";

h1 {
  margin: 0;
}
.top {
  position: sticky;
  top: -5px;
  z-index: 500;
  padding: 5px;
  background-color: $fg-h;
  border-radius: 3px 3px 5px 5px;
  margin-bottom: 20px;
}
.search-bar {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;



  button,
  .search-input-bar {
    border: none;
    background-color: $fg-a;
    transition: 150ms all;
    color: $txt-m;
    font-size: 1.2rem;
    border-radius: 5px;
    padding: 2px 5px;
    box-sizing: border-box;

    &:hover {
      background-color: $fg-ma;
    }
  }

  .search-input-bar {
    padding: 5px 8px;
    width: min(30vw, 400px);
  }

  .filter-button {
    margin-left: 5px;
    margin-bottom: -50px;
    &.active {
      background-color: $main-am;
    }
  }
  .pages {
    button {
      width: 35px;
      aspect-ratio: 1/1;
      cursor: pointer;
    }
  }
}
.mod-grid {
  display: flex;
  flex-direction: column;
  gap: 20px;

  .mod-box {
    h2 {
      margin: 6px 0;
    }
    p {
      margin: 5px 0;
    }
    display: grid;
    grid-template-columns: 150px 3fr 1fr;
    gap: 10px;
    padding: 1rem;
    overflow: hidden;
    border-radius: $brr;
    background-color: $fg-a;
    position: relative;
    font-size: 1.3rem;
    cursor: pointer;

    svg,
    img {
      background-color: $fg-ma;
      border-radius: $brr;
    }
    &.downloaded {
      outline: 2px solid $main;
    }

    .mod-info {
      .tags {
        display: flex;
        flex-wrap: wrap;
        gap: 5px;
        span {
          padding: 0 3px 3px;
          font-variant: small-caps;
          background: $main-am;
          border-radius: 5px;
        }
      }
    }

    .mod-num {
      margin-top: 20px;
      justify-self: flex-end;
      display: grid;
      grid-template-rows: 38px 38px;

      span {
        justify-self: flex-end;
        background-color: $fg-ma;
        margin-bottom: 5px;
        height: fit-content;
        border-radius: 10px;
        padding: 5px;
        box-sizing: border-box;

        display: flex;
        justify-content: end;
        width: fit-content;
      }
    }
  }
}
</style>

<template>
  <div class="all-mods" ref="allMods" v-show="openModId == ''">
    <div class="top">
      <h1>MODS</h1>
      <hr>
      <div class="search-bar">
        <div class="search-input">
          <input
            type="text"
            class="search-input-bar"
            placeholder="Search mods..."
            v-model="modQuery"
            @input="searchUpdate"
          />
          <button
            class="filter-button"
            :class="{active: useFilters}"
            @click="
              () => {
                useFilters = !useFilters;
                fetchModsFromModrinth();
              }
            "
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="22"
              viewBox="0 0 24 24"
              v-if="useFilters"
            >
              <path
                fill="currentColor"
                d="M14 12v7.88c.04.3-.06.62-.29.83a.996.996 0 0 1-1.41 0l-2.01-2.01a.989.989 0 0 1-.29-.83V12h-.03L4.21 4.62a1 1 0 0 1 .17-1.4c.19-.14.4-.22.62-.22h14c.22 0 .43.08.62.22a1 1 0 0 1 .17 1.4L14.03 12H14Z"
              />
            </svg>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="22"
              viewBox="0 0 24 24"
              v-else
            >
              <path
                fill="currentColor"
                d="M2.39 1.73L1.11 3l8.39 8.37l.47.63H10v5.87c-.04.29.06.6.29.83l2.01 2.01c.39.39 1.02.39 1.41 0c.23-.21.33-.53.29-.83v-3.99l6.84 6.84l1.27-1.27L14 13.35L9.41 8.76L4.15 3.5L2.39 1.73M6.21 3l8.33 8.34l5.25-6.72a1 1 0 0 0-.17-1.4c-.19-.14-.4-.22-.62-.22H6.21Z"
              />
            </svg>
          </button>
        </div>
        <div class="pages">
          <button @click="goToPrevPage">&lt;</button>
          {{ offset / pageLenght + 1 }}
          <button @click="goToNextPage">&gt;</button>
        </div>
      </div>
    </div>
    <div class="mod-grid">
      <article
        class="mod-box"
        :class="{downloaded: downloadedModIds.indexOf(mod.project_id) !== -1}"
        v-for="(mod, index) in modData"
        :key="index"
        @click="goToModInfo(mod.project_id)"
      >
        <img
          :key="mod.icon_url"
          v-if="mod.icon_url"
          :src="mod.icon_url"
          width="150"
          height="150"
          alt=""
        />
        <svg
          v-else
          xml:space="preserve"
          fill-rule="evenodd"
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-miterlimit="1.5"
          viewBox="0 0 104 104"
          width="150"
          height="150"
          aria-hidden="true"
        >
          <path fill="none" d="M0 0h103.4v103.4H0z" />
          <path
            fill="none"
            stroke="#9a9a9a"
            stroke-width="5"
            d="M51.7 92.5V51.7L16.4 31.3l35.3 20.4L87 31.3 51.7 11 16.4 31.3v40.8l35.3 20.4L87 72V31.3L51.7 11"
          />
        </svg>

        <div class="mod-info">
          <h2>{{ mod.title }}</h2>
          <p>{{ mod.description }}</p>
          <div class="tags">
            <span v-for="(category, index) in mod.categories" :key="index">{{
              category
            }}</span>
          </div>
        </div>
        <div class="mod-num">
          <span
            >{{ mod.downloads }}
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
            >
              <path
                fill="currentColor"
                d="M5 20h14v-2H5m14-9h-4V3H9v6H5l7 7l7-7Z"
              /></svg></span>
          <span
            >{{ mod.follows }}
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
            >
              <path
                fill="currentColor"
                d="m12.1 18.55l-.1.1l-.11-.1C7.14 14.24 4 11.39 4 8.5C4 6.5 5.5 5 7.5 5c1.54 0 3.04 1 3.57 2.36h1.86C13.46 6 14.96 5 16.5 5c2 0 3.5 1.5 3.5 3.5c0 2.89-3.14 5.74-7.9 10.05M16.5 3c-1.74 0-3.41.81-4.5 2.08C10.91 3.81 9.24 3 7.5 3C4.42 3 2 5.41 2 8.5c0 3.77 3.4 6.86 8.55 11.53L12 21.35l1.45-1.32C18.6 15.36 22 12.27 22 8.5C22 5.41 19.58 3 16.5 3Z"
              /></svg></span>
        </div>
      </article>
    </div>
  </div>
  <ModInfo
    :modId="openModId"
    v-if="serverVersion && serverType"
    :key="openModId + serverVersion + serverType + serverId + Date.now()"
    :serverType="serverType"
    :applyFilters="useFilters"
    :serverId="serverId"
    :serverVersion="serverVersion"
    @goToModGrid="goToModGrid"
    v-show="openModId != ''"
  ></ModInfo>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import ModInfo from "./modInfo.vue";
export default {
  name: "ModrinthMods",
  data() {
    return {
      modData: [],
      downloadedModIds: [],
      downloadedMods: [],
      offset: 0,
      pageLenght: 20,
      openModId: "",
      modQuery: "",
      searchTimeout: 0,
      useFilters: true,
    };
  },
  components: {
    ModInfo,
  },
  props: {
    serverId: {
      default: "",
      type: String,
    },
    serverType: {
      default: "",
      type: String,
    },
    serverVersion: {
      default: "",
      type: String,
    },
  },
  mounted() {
    this.fetchModsFromModrinth();
    this.getDownloadedMods();
  },
  methods: {
    async getDownloadedMods() {
      await invoke("get_mods_as_string",{
          serverId: this.serverId
      }).then(downloadedMods => {
        this.downloadedMods = JSON.parse(downloadedMods)
        this.downloadedModIds = JSON.parse(downloadedMods).map(item => item.project_id);
        console.log(this.downloadedModIds);
      }) 
    },
    fetchModsFromModrinth() {
      if (
        this.serverId != "" &&
        this.serverType != "" &&
        this.serverVersion != ""
      ) {
        let url = `https://api.modrinth.com/v2/search`;
        url = new URL(url);
        let params = new URLSearchParams();
        if (this.useFilters) {
          params.append(
            "filters",
            `versions="${this.serverVersion}" AND project_type="mod" AND(server_side="required" OR server_side="optional") AND (categories="${this.serverType}"${this.serverType === "quilt"?" OR categories='fabric'":""})`
          );
        }
        params.append("offset", this.offset);
        params.append("limit", this.pageLenght);
        if (this.modQuery != "") {
          params.append("query", this.modQuery);
        }
        url.search = params.toString();
        console.log(url.toString());
        fetch(url, {
          headers: {
            "user-agent":
              "Mozilla/5.0 (Android 13; Mobile; LG-M255; rv:114.0) Gecko/114.0 Firefox/114.0",
          },
        })
          .then((response) => {
            return response.json(); // return the promise
          })
          .then((data) => {
            this.modData = data.hits;
            console.log(this.modData);
          })
          .catch((e) => {
            console.error(e);
          });
      }
    },
    searchUpdate() {
      clearTimeout(this.searchTimeout);
      this.searchTimeout = setTimeout(() => {
        this.modData = {};
        this.fetchModsFromModrinth();
      }, 300);
    },
    goToModInfo(modId) {
      this.openModId = modId;
      console.log(this.openModId);
    },
    goToModGrid() {
      this.openModId = "";
      this.getDownloadedMods();
    },
    goToNextPage() {
      this.modData = {};
      this.offset += this.pageLenght;
      this.fetchModsFromModrinth();
    },
    goToPrevPage() {
      this.modData = {};
      this.offset = Math.max(0, this.offset - this.pageLenght);
      this.fetchModsFromModrinth();
    },
  },
};
</script>

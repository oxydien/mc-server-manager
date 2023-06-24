<style lang="scss" scoped>
@import "../../assets/scss/global.scss";
.mod-bar {
  display: grid;
  grid-template-columns: 50px 10fr 120px;
  padding: 5px;
  margin: -5px;
  background-color: $fg-h;
  position: sticky;
  top: -5px;
  z-index: 500;
  border-radius: 3px 3px 5px 5px;
  border-bottom: 1px solid $fg-a;

  .back-button {
    cursor: pointer;
  }

  h1 {
    margin: 0;
    font-size: 28px;
  }

  button {
    display: flex;
    align-items: center;
    font-family: Arial, Helvetica, sans-serif;
    font-size: 1.1rem;
    margin: 0 5px 0 0;
    overflow: hidden;

    color: $txt;
    background-color: $main;
    padding: 0 0.5rem;
    border: none;
    border-radius: 6px;
    box-sizing: border-box;

    cursor: pointer;
    &:disabled {
        background-color: $main-m;
        cursor: not-allowed;
    }
  }
}

.mod-body {
  padding: 1rem;
  img,
  svg {
    float: right;
    margin: 20px;
    background-color: $fg-a;
    border-radius: $brr;
  }
}
.mod-info {
  position: relative;
}
.modal-download {
  position: absolute;
  top: 0;
  width: 100%;
  height: max(100%, 90vh);
  background-color: $fg-ma;
  .modal-window {
    width: 300px;
    position: sticky;
    left: calc(50vw - 390px);
    top: 30vh;
    transform: translateY(-50vh), translateX(-50%);
    padding: 1rem;
    background-color: $fg-a;
    border-radius: $brr;

    h3 {
      display: flex;
      justify-content: space-between;
      margin: 3px 0;
      font-size: 1.3rem;

      button {
        border: none;
        border-radius: 5px;
        height: 30px;
        color: $txt;
        width: 30px;
        background-color: $fg-ma;
        transition: all 200ms;
        cursor: pointer;

        &:hover {
          background-color: $fg-m;
        }
      }
    }

    select {
      border: none;
      padding: 3px 1rem;
      font-size: 1.2rem;
      width: 100%;
      background-color: $fg-ma;
      border-radius: 5px;
      color: $txt;
      cursor: pointer;

      option {
        background: $fg;
        &:hover {
          background: $fg-h;
        }
      }
    }

    .loaders-versions {
      display: flex;
      flex-direction: column;
      gap: 5px;
      margin-top: 5px;
      padding: 2px 0;
      overflow: auto;
      span {
        padding: 2px;
        margin: 8px 3px 0 0;
        border-radius: 5px;
        background-color: $fg-h;
        &.tag-loader {
          background-color: $main-am;
        } 
        &.tag-version {
          background-color: $main-m;
        } 
      }

      
    }

    .changelog {
      height: 200px;
      overflow: auto;
    }

    .download-button {
      border: none;
      padding: 8px 1rem;
      font-size: 1.2rem;
      width: 100%;
      background-color: $main;
      border-radius: 5px;
      transition: all 200ms;
      color: $txt;
      cursor: pointer;

      &:disabled {
        background-color: $main-m;
        cursor: not-allowed;
      }
      &:hover {
        background-color: $main-m;
      }
    }
  }
}
</style>

<template>
  <div class="mod-info">
    <div class="mod-bar">
      <div class="back-button" @click="showDownload = false;$emit('goToModGrid')" v-if="modId != ''">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="30"
          height="30"
          viewBox="0 0 512 512"
        >
          <path
            fill="currentColor"
            d="M321.94 98L158.82 237.78a24 24 0 0 0 0 36.44L321.94 414c15.57 13.34 39.62 2.28 39.62-18.22v-279.6c0-20.5-24.05-31.56-39.62-18.18Z"
          />
        </svg>
      </div>
      <h1>{{ modData.title + " " + modData.id || "Loading..." }}</h1>
      <button @click="showDownload = true" :key="modData" :disabled="modData.modVersions === null">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        >
          <path
            fill="currentColor"
            d="M5 20h14v-2H5m14-9h-4V3H9v6H5l7 7l7-7Z"
          />
        </svg>
        <span>Download</span>
      </button>
    </div>
    <div class="mod-body" v-if="modData != {}">
      <img
        v-if="modData.icon_url"
        :src="modData.icon_url"
        width="200"
        height="200"
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
        width="200"
        height="200"
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
      <vue-markdown :source="modData.body || ''" />
    </div>
    <div class="modal-download" :key="modData"  v-if="showDownload && modData.modVersions !== null">
      <div class="modal-window">
        <h3>
          <span>{{ modData.title }}</span>
          <button @click="showDownload = false">X</button>
        </h3>
        <select @change="selectVersion($event)" :key="modData.modVersions + modData + showDownload">
          <option
            v-for="(version, index) in modData.modVersions"
            :value="version.id"
            :key="version.id + index"
          >
            {{ version.version_number || version.id }}
          </option>
        </select> 
        <div class="loaders-versions">
          <div class="loaders">
            <span class="tag-loader" v-for="(loader, index) in versionLoaders" :key="index">{{ loader }}</span>
          </div>
          <div class="versions">
            <span class="tag-version" v-for="(version, index) in versionVersions" :key="index">{{ version }}</span>
          </div>
        </div>
        <hr />
        <div class="changelog">
          <vue-markdown :source="versionText" />
        </div>
        <button
          class="download-button"
          @click="downloadVersion"
          ref="downloadButton"
          :key="modData.modVersions"
          :disabled="!modData.modVersions || modData.modVersions.length < 1"
        >
          Download {{ versionId }}
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import VueMarkdown from "vue-markdown-render";
export default {
  name: "ModInfo",
  props: {
    modId: {
      default: "",
      type: String,
    },
    applyFilters: {
      default: "",
      type: Boolean,
    },
    serverType: {
      default: "",
      type: String,
    },
    serverId: {
      default: "",
      type: String,
    },
    serverVersion: {
      default: "",
      type: String,
    },
  },
  components: {
    VueMarkdown,
  },
  data() {
    return {
      modData: {},
      versionText: "",
      versionVersions: [],
      versionLoaders: [],
      showDownload: false,
      versionId: "",
    };
  },
  mounted() {
    this.fetchModInfo();
    this.fetchModVersions();
  },
  emits: ["goToModGrid"],
  methods: {
    async fetchModInfo() {
      if (this.modId != "") {
        await fetch(`https://api.modrinth.com/v2/project/${this.modId}`, {
          headers: {
            "user-agent":
              "Mozilla/5.0 (Android 13; Mobile; LG-M255; rv:114.0) Gecko/114.0 Firefox/114.0",
          },
        })
          .then((response) => {
            return response.json(); // return the promise
          })
          .then((data) => {
            this.modData = data;
            console.log("modData",this.modData);
          })
          .catch((e) => {
            console.error(e);
          });
      }
    },
    async fetchModVersions() {
      if (this.modId != "") {
        let url = `https://api.modrinth.com/v2/project/${this.modId}/version`;
        url = new URL(url);
        let params = new URLSearchParams();
        if (this.applyFilters) {
          params.append("game_versions", `["${this.serverVersion}"]`);
          params.append("loaders", `["${this.serverType}"${this.serverType === "quilt"?',"fabric"':''}]`);
        }
        url.search = params.toString();
        console.log("modUrl",url);
        await fetch(url, {
          headers: {
            "user-agent":
              "Mozilla/5.0 (Android 13; Mobile; LG-M255; rv:114.0) Gecko/114.0 Firefox/114.0",
          },
        })
          .then((response) => {
            return response.json();
          })
          .then((data) => {
            this.modData.modVersions = data;
          })
          .then(() => {
            if (this.modData.modVersions[0]) {
              this.versionText = this.modData.modVersions[0].changelog;
              this.versionVersions = this.modData.modVersions[0].game_versions;
              this.versionLoaders = this.modData.modVersions[0].loaders;
              this.versionId = this.modData.modVersions[0].id;
            }
            console.log("ModVersions",this.modData.modVersions);
          })
          .catch((e) => {
            console.error(e);
          });
      }
    },
    async downloadVersion() {
      const url = this.modData.modVersions.find(
        (obj) => obj.id === this.versionId
      ).files[0].url;
      const modInfo = {
        project_id: this.modData.id,
        version_id: this.versionId,
        name: this.modData.title,
        file_name: this.modData.modVersions[0].files[0].filename,
      }
      this.$refs["downloadButton"].disabled = true;
        await invoke("download_mod_command", {
          serverId: this.serverId,
          modUrl: url,
          modInfo: JSON.stringify(modInfo),
        }).then(downloadingMod => {
          if (downloadingMod.startsWith("[success]")) {
            this.$refs["downloadButton"].innerText = "Success";
          }
          console.log(downloadingMod);
        })
    },
    selectVersion(event) {
      this.versionText = this.modData.modVersions.find(
        (obj) => obj.id === event.target.value
      ).changelog;
      this.versionVersions = this.modData.modVersions.find(
        (obj) => obj.id === event.target.value
      ).game_versions;
      this.versionLoaders = this.modData.modVersions.find(
        (obj) => obj.id === event.target.value
      ).loaders;
      this.versionId = event.target.value;
    },
  },
};
</script>

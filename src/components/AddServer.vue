<style lang="scss" scoped>
@import "../assets/scss/global.scss";
main {
  background-color: $main-am !important;
  opacity: 0.8;
  background-image: linear-gradient(
      30deg,
      $main 12%,
      transparent 12.5%,
      transparent 87%,
      $main 87.5%,
      $main
    ),
    linear-gradient(
      150deg,
      $main 12%,
      transparent 12.5%,
      transparent 87%,
      $main 87.5%,
      $main
    ),
    linear-gradient(
      30deg,
      $main 12%,
      transparent 12.5%,
      transparent 87%,
      $main 87.5%,
      $main
    ),
    linear-gradient(
      150deg,
      $main 12%,
      transparent 12.5%,
      transparent 87%,
      $main 87.5%,
      $main
    ),
    linear-gradient(
      60deg,
      $main-a 25%,
      transparent 25.5%,
      transparent 75%,
      $main-a 75%,
      $main-a
    ),
    linear-gradient(
      60deg,
      $main-a 25%,
      transparent 25.5%,
      transparent 75%,
      $main-a 75%,
      $main-a
    ),
    linear-gradient(90deg, #0faee4a5, #11111154);
  background-size: 60px 108px;
  background-position: 0 0, 0 0, 30px 54px, 30px 54px, 0 0, 30px 54px, 0 0;
  box-sizing: border-box;
  animation: backround_animation 500s linear infinite;

  @keyframes backround_animation {
    0% {
      background-position: 0 0, 0 0, 30px 54px, 30px 54px, 0 0, 30px 54px, 0 0;
    }
    100% {
      background-position: calc(0px - 6000px) 1080px, calc(0px - 6000px) 1080px,
        calc(30px - 6000px) calc(54px + 1080px),
        calc(30px - 6000px) calc(54px + 1080px), calc(0px - 6000px) 1080px,
        calc(30px - 6000px) calc(54px + 1080px), calc(0px - 6000px) 1080px;
    }
  }
}

.wrapper {
  padding: 5px;
  overflow: hidden;
  position: relative;

  .pages {
    .page {
      .loader-versions label,
      & > label {
        display: block;
        font-weight: 600;
        font-size: 1.15rem;
        margin: 8px 0 5px 0;
      }
      .eula,
      .image-input {
        display: flex;
        gap: 10px;
        padding: 10px;
        background-color: rgba(67, 73, 86, 0.42);

        .inputs {
          display: flex;
          flex-flow: column nowrap;
          justify-content: space-between;
        }
      }
      .eula {
        justify-content: space-between;
      }
      .show-snapshots {
        display: inline-flex;
        margin-left: 5px;
        button {
          border: none !important;
          appearance: none !important;
        }
      }

      &.information {
        span {
          display: inline-block;
          font-size: 1.2rem;
          font-weight: 600;
          margin: 5px 0;
        }
      }
    }
  }

  .navigation {
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-sizing: border-box;
    padding-top: 5px;
    margin-top: 5px;
    border-top: 1px solid $bg;
  }
}
</style>

<template>
  <main>
    <Modal :closable="false" :header="getModalHeader" ref="modal">
      <div class="wrapper">
        <div
          class="pages"
          v-if="!creatingServer && !successfullyCreated && !failedToCreate"
        >
          <div v-show="currentPage === 1" class="page">
            <label for="serverName" style="margin-top: 0">Server name</label>
            <div class="iconified-input">
              <GlobeIcon />
              <input
                v-model="serverName"
                type="text"
                id="serverName"
                placeholder="Server name..."
                maxlength="32"
              />
              <Button @click="() => (serverName = '')">
                <XIcon />
              </Button>
            </div>
            <label for="serverIcon">Server icon</label>
            <Card class="image-input">
              <Avatar size="md" :src="imageSrc"></Avatar>
              <div class="inputs">
                <FileInput
                  :max-size="2356221"
                  accept="image/png,image/jpeg,image/gif,image/webp"
                  class="btn"
                  :prompt="imagePrompt"
                  @change="updateImage"
                >
                  <UploadIcon />
                </FileInput>
                <div class="iconified-input">
                  <LinkIcon />
                  <input
                    v-model="imageSrc"
                    type="text"
                    id="serverIcon"
                    placeholder="Image url..."
                  />
                  <Button @click="() => (imageSrc = '')">
                    <XIcon />
                  </Button>
                </div>
              </div>
            </Card>
            <label for="serverType">Server type</label>
            <Chips
              v-model="serverType"
              name="serverType"
              @update:modelValue="serverTypeChange"
              :items="['vanilla', 'quilt', 'fabric']"
              placeholder="Choose server type..."
            />
            <label for="serverVersion">Server version</label>
            <DropdownSelect
              v-model="serverVersion"
              name="serverVersion"
              :options="filteredVersions"
              placeholder="Choose server type..."
              render-up
            />
            <Checkbox class="show-snapshots" v-model="showSnapshots"
              >Show snapshots</Checkbox
            >
            <div v-if="serverType !== 'vanilla'" class="loader-versions">
              <label for="serverType">Loader version</label>
              <DropdownSelect
                v-model="loaderVersion"
                v-if="serverType === 'fabric'"
                name="loaderVersion"
                :options="fabricVersions"
                placeholder="Choose server type..."
                render-up
              />
              <DropdownSelect
                v-model="loaderVersion"
                v-if="serverType === 'quilt'"
                name="loaderVersion"
                :options="quiltVersions"
                placeholder="Choose server type..."
                render-up
              />
            </div>
          </div>
          <div class="page information" v-if="currentPage === 2">
            <Avatar size="md" style="float: right" :src="imageSrc"></Avatar>
            <span>Server name</span>: {{ serverName }}<br />
            <span>Minecraft version</span>: {{ serverVersion }}<br />
            <span>Server type</span>: {{ serverType }}<br />
            <div v-if="serverType !== 'vanilla'">
              <span>Loader version</span>: {{ loaderVersion }}
            </div>
            <Card class="eula">
              <span
                >Do you agree with mojang's
                <a href="https://www.minecraft.net/en-us/eula">eula</a></span
              >
              <Toggle
                v-model="agreeToEula"
                :checked="agreeToEula"
                id="eulaButton"
              ></Toggle>
            </Card>
          </div>
        </div>
        <div v-else-if="creatingServer">
          <LoadingCube></LoadingCube>
        </div>
        <div v-else-if="successfullyCreated">
          <Card
            ><Badge color="green" type="Success" /> Successfully created server
            {{ serverName }}</Card
          >
        </div>
        <div v-else-if="failedToCreate">
          <Card
            ><Badge color="red" type="fail" /> Failed to create server
            {{ serverName }}</Card
          >
          {{ errorMessage }}
        </div>
        <div
          class="navigation"
          v-if="!creatingServer && !successfullyCreated && !failedToCreate"
        >
          <Button
            color="secondary"
            @click="prevPage"
            iconOnly
            :disabled="currentPage <= 1"
          >
            <LeftArrowIcon></LeftArrowIcon>
          </Button>
          {{ currentPage }}
          <Button
            color="primary"
            @click="nextPage"
            :disabled="serverName === '' || (currentPage === 2 && !agreeToEula)"
            :iconOnly="currentPage !== 2"
          >
            <RightArrowIcon></RightArrowIcon>
            <span v-if="currentPage === 2">Create</span>
          </Button>
        </div>
      </div>
    </Modal>
    <Notifications ref="notifsContainer" />
  </main>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import LoadingCube from "./loading.vue";
import {
  FileInput,
  Badge,
  Card,
  Checkbox,
  Avatar,
  DropdownSelect,
  Chips,
  GlobeIcon,
  Toggle,
  LinkIcon,
  UploadIcon,
  XIcon,
  Button,
  Modal,
  RightArrowIcon,
  Notifications,
  LeftArrowIcon,
} from "omorphia";
export default {
  name: "AddServer",
  data() {
    return {
      currentPage: 1,
      serverName: "",
      imageSrc: "",
      imagePrompt: "Upload icon",
      minecraftVersions: [],
      quiltVersions: [],
      fabricVersions: [],
      serverVersion: "",
      loaderVersion: "",
      serverType: "vanilla",
      showSnapshots: false,
      agreeToEula: false,
      creatingServer: false,
      successfullyCreated: false,
      failedToCreate: false,
      errorMessage: "",
    };
  },
  components: {
    Avatar,
    Badge,
    Card,
    FileInput,
    LoadingCube,
    DropdownSelect,
    Checkbox,
    RightArrowIcon,
    LeftArrowIcon,
    Chips,
    Toggle,
    UploadIcon,
    GlobeIcon,
    LinkIcon,
    XIcon,
    Button,
    Notifications,
    Modal,
  },
  mounted() {
    this.currentPage = 1;
    this.openServerModal();
    this.loadMinecraftVersions();
    this.loadQuiltVersions();
    this.loadFabricVersions();
  },
  methods: {
    openServerModal() {
      this.$refs.modal.show();
    },
    prevPage() {
      this.currentPage = Math.max(1, this.currentPage - 1);
    },
    nextPage() {
      if (this.currentPage === 2) {
        this.createServer();
      }
      this.currentPage = Math.min(2, this.currentPage + 1);
    },
    updateImage(event) {
      const file = event[0];
      const fileName = file.name;
      const shortenedName = this.shortenFileName(fileName, 18);

      this.imagePrompt = shortenedName;

      const reader = new FileReader();
      reader.onload = (e) => {
        this.imageSrc = e.target.result;
      };
      reader.readAsDataURL(file);
    },

    shortenFileName(fileName, maxLength) {
      if (fileName.length <= maxLength) {
        return fileName;
      }

      const baseNameLength = maxLength;
      const firstCharacters = fileName.substring(
        0,
        Math.ceil(baseNameLength / 2)
      );
      const lastCharacters = fileName.substring(
        fileName.length - Math.floor(baseNameLength / 2)
      );
      const shortenedName = firstCharacters + "..." + lastCharacters;

      return shortenedName;
    },
    loadMinecraftVersions() {
      fetch("https://launchermeta.mojang.com/mc/game/version_manifest.json")
        .then((response) => response.json())
        .then((data) => {
          this.minecraftVersions = data.versions;
          this.serverVersion = data.latest.release;
        })
        .catch((error) => {
          console.log(
            "An error occurred while fetching Minecraft versions:",
            error
          );
        });
    },
    loadQuiltVersions() {
      fetch(
        "https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-loader/maven-metadata.xml"
      )
        .then((response) => response.text())
        .then((xmlText) => {
          const parser = new DOMParser();
          const xmlDoc = parser.parseFromString(xmlText, "application/xml");
          const versionNodes = xmlDoc.getElementsByTagName("version");
          const quiltVersions = Array.from(versionNodes).map(
            (node) => node.textContent
          );

          this.quiltVersions = quiltVersions.reverse();
        })
        .catch((error) => {
          console.error("Error loading Quilt versions:", error);
        });
    },
    loadFabricVersions() {
      fetch(
        "https://maven.fabricmc.net/net/fabricmc/fabric-loader/maven-metadata.xml"
      )
        .then((response) => response.text())
        .then((xmlText) => {
          const parser = new DOMParser();
          const xmlDoc = parser.parseFromString(xmlText, "application/xml");
          const versionNodes = xmlDoc.getElementsByTagName("version");
          const fabricVersions = Array.from(versionNodes).map(
            (node) => node.textContent
          );

          this.fabricVersions = fabricVersions.reverse();
        })
        .catch((error) => {
          console.error("Error loading Quilt versions:", error);
        });
    },
    addNotification(type,text) {
      this.$refs.notifsContainer.addNotification({
        title: "Server Creation",
        text: text,
        type,
      });
    },
    serverTypeChange() {
      if (this.serverType === "quilt")
        this.loaderVersion = this.quiltVersions[0];
      if (this.serverType === "fabric")
        this.loaderVersion = this.fabricVersions[0];
    },
    async createServer() {
      this.creatingServer = true;
      let input = {
        name: this.serverName,
        serverType: this.serverType.toLowerCase(),
        mcVersion: this.serverVersion,
        image: this.imageSrc,
      };// The loader version is not implemented in the Rust code
      console.log(input);
      await invoke("add_server_command", input).then((response) => {
        console.log(response);
        if (response.startsWith("[success]")) {
          this.creatingServer = false;
          this.successfullyCreated = true;
          this.addNotification('success',response)
        } else {
          this.creatingServer = false;
          this.failedToCreate = true;
          this.errorMessage = response;
          this.addNotification('error',response)
        }
      });
    },
  },
  computed: {
    getModalHeader() {
      if (this.creatingServer) return "Create server - creating";
      else if (this.currentPage === 1) return "Create server - info";
      else if (this.currentPage === 2)
        return "Create server - " + this.serverName;
      else return "Create server - ??";
    },
    filteredVersions() {
      if (this.showSnapshots) {
        return this.minecraftVersions.map((version) => version.id);
      }
      return this.minecraftVersions
        .filter((version) => version.type === "release")
        .map((version) => version.id);
    },
  },
};
</script>

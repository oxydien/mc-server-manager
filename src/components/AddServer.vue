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
        calc(30px - 6000px) calc(54px + 1080px), calc(30px - 6000px) calc(54px + 1080px),
        calc(0px - 6000px) 1080px, calc(30px - 6000px) calc(54px + 1080px), calc(0px - 6000px) 1080px;
    }
  }

  .wrapper {
    position: relative;
    top: 40%;
    transform: translateY(-50%);
  }
  .form {
    background-color: $fg-h;
    width: fit-content;
    padding: 1rem;
    border-radius: $brr;
    margin: 0 auto;
    min-width: 200px;
  }
  h2 {
    font-size: clamp(0.8rem, 5vw, 3rem);
    margin: 0;
    text-align: center;
    text-shadow: 0 0 10px black;
  }
  select,
  button,
  input {
    font-family: Arial, Helvetica, sans-serif;
    font-size: 1.1rem;
    display: block;
    margin: 10px auto 0 auto;
    width: 100%;
    overflow: hidden;

    color: $txt;
    background-color: $fg-a;
    padding: 0.505rem 0.808rem;
    border: none;
    border-radius: 6px;
    box-sizing: border-box;
  }
  select,
  button {
    cursor: pointer;
  }

  .image-input {
    width: fit-content;
    margin: 0 auto;
    display: flex;
    justify-content: center;
    gap: 20px;
    border-radius: $brr;
    background-color: $fg-a;

    padding: 10px;
    input,
    button {
      margin: 0;
      width: 200px;
      background-color: $fg-ma;
    }
    .image-container {
      width: 100px;
      height: 100px;
      background-color: $fg-h;
      border-radius: $brr;
      position: relative;

      &::after {
        content: "";
        position: absolute;
        inset: 0;
        width: 100%;
        height: 100%;
        background-color: $bg-a;
        background-repeat: no-repeat;
        background-size: cover;
        transition: opacity 150ms;
        background-image: url(data:image/svg+xml,%3Csvg%20data-v-7a7a37b1%3D%22%22%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%2230%22%20height%3D%2230%22%20viewBox%3D%220%200%2024%2024%22%20style%3D%22width%3A%20100%25%3B%20height%3A%20100%25%3B%22%3E%3Cpath%20data-v-7a7a37b1%3D%22%22%20fill%3D%22white%22%20d%3D%22M19%2013h-6v6h-2v-6H5v-2h6V5h2v6h6v2Z%22%3E%3C%2Fpath%3E%3C%2Fsvg%3E);
        border-radius: $brr;
        opacity: 0;
      }

      &:hover {
        &::after {
          opacity: 1;
        }
        svg {
          color: $main-h;
        }
      }

      img,
      svg {
        object-fit: cover;
        width: 100%;
        height: 100%;
        border-radius: $brr;
      }
    }
  }
  .submit-button {
    transition: all 150ms cubic-bezier(0.215, 0.61, 0.355, 1);
    background-color: $main-am;
    &:hover {
      background-color: $main-m;
    }
    &:disabled {
      background-color: $main-m;
      cursor: not-allowed;
    }
  }
}
.server-info {
  display: flex;
  align-items: center;
  gap: 10px;
  overflow: hidden;
  background-color: $fg-a;
  padding: 0.5rem;
  box-sizing: border-box;
  border-radius: $brr;
  margin-bottom: 10px;
  cursor: pointer;
  svg,
  img {
    width: 50px;
    height: 50px;
    background-color: $fg-ma;
    border-radius: $brr;
    object-fit: cover;
  }
  p {
    white-space: nowrap;
    text-overflow: ellipsis;
  }
}
</style>

<template>
  <main>
    <div class="wrapper">
      <h2>Create new server</h2>
      <div class="form" v-if="creatingServer">
        <h3 style="text-align: center; margin-top: 0">
          Creating
          <code
            style="
              background-color: #0000002a;
              border-radius: 5px;
              padding: 2px;
            "
            >{{ serverName }}</code
          >
          server.
        </h3>
        <LoadingCube></LoadingCube>
      </div>
      <div class="form" v-else-if="failedToCreate">
        <h3>There was error when creating your server:</h3>
        <p style="max-width: 500px">{{ errorMessage }}</p>
      </div>
      <div class="form" v-else-if="successfullyCreated">
        <h3>Successfully created your server</h3>
        <div class="server-info" style="cursor: default">
          <div class="image-holder">
            <img v-if="imageSrc" :src="imageSrc" alt="" />
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
          <p :title="serverName">
            {{ serverName }}
          </p>
        </div>
      </div>
      <section class="form" v-else>
        <div v-show="page === 1">
          <label for="serverName"> Name: </label>
          <input
            placeholder="Server name..."
            maxlength="50"
            style="margin-bottom: 10px"
            type="text"
            id="serverName"
            v-model="serverName"
            autocomplete="off"
          />
          <span> Icon: </span>
          <div class="image-input">
            <div
              class="image-container"
              style="cursor: pointer"
              @click="openFileWindow"
            >
              <img
                :src="imageSrc"
                v-if="imageSrc"
                @error="replaceWithSVG"
                :key="imageSrc"
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
            <div class="inputs">
              <button
                id="imageFileButton"
                ref="imageFileButton"
                @click="openFileWindow"
              >
                Click to upload a file
              </button>

              <div
                style="
                  height: 10px;
                  width: 0px;
                  overflow: hidden;
                  visibility: hidden;
                "
              >
                <input
                  ref="imageFileInput"
                  type="file"
                  accept="image/*"
                  @change="updateImage"
                />
              </div>
              <input
                type="text"
                ref="imageURLInput"
                placeholder="Image URL"
                @change="imageURL"
              />
            </div>
          </div>
        </div>
        <div v-show="page === 2">
          <div class="server-info" @click="prevPage">
            <div class="image-holder">
              <img v-if="imageSrc" :src="imageSrc" alt="" />
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
            <p :title="serverName">
              {{ serverName }}
            </p>
          </div>
          <span>Server type:</span>
          <select
            name="serverType"
            v-model="serverType"
            id="serverType"
            ref="serverType"
          >
            <option value="vanilla">Vanilla</option>
            <option value="quilt">Quilt</option>
            <option value="fabric">Fabric</option>
          </select>
          <span style="margin-top: 10px; display: block"
            >Minecraft version:</span
          >
          <div style="display: flex">
            <select
              name="serverVersion"
              id="serverVersion"
              ref="serverVersion"
              style="display: inline-block"
              v-model="serverVersion"
              :key="showSnapshots"
            >
              <option
                :value="version.id"
                v-for="(version, index) in filteredVersions()"
                :key="index"
              >
                {{ version.id }}
              </option>
            </select>
            <label
              style="margin: 20px 10px 0; white-space: nowrap"
              for="showSnapshots"
              >Show snapshots</label
            >
            <input
              type="checkbox"
              style="display: inline; margin: 10px 0 0; width: 30px"
              id="showSnapshots"
              v-model="showSnapshots"
            />
          </div>
        </div>
        <div v-show="page === 3">
          <div class="server-info" @click="prevPage">
            <div class="image-holder">
              <img v-if="imageSrc" :src="imageSrc" alt="" />
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
            <p :title="serverName">
              {{ serverName }}
            </p>
          </div>
          <p>
            Server type: <b>{{ serverType }}</b>
          </p>
          <p>
            Minecraft version: <b>{{ serverVersion }}</b>
          </p>
          <p>
            <label for="agreeToEula">Agree to mojang's Eula: </label
            ><input
              id="agreeToEula"
              type="checkbox"
              style="display: inline; margin: 0; width: 15px; height: 15px"
              v-model="agreeToEula"
            />
          </p>
          <hr />
          <span>Do you want to create this server?</span>
        </div>
        <button
          type="submit"
          ref="submitButton"
          @click="nextPage"
          class="submit-button"
          :disabled="
            (page === 1 && serverName === '') || (page === 3 && !agreeToEula)
          "
        >
          {{ page != 3 ? "Next.." : "Yes - CREATE IT!" }}
        </button>
      </section>
    </div>
  </main>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import LoadingCube from "./loading.vue";
export default {
  name: "AddServer",
  data() {
    return {
      serverName: "",
      imageSrc: "",
      page: 1,
      minecraftVersions: [],
      serverVersion: "",
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
    LoadingCube,
  },
  mounted() {
    this.page = 1;
    this.loadMinecraftVersions();
  },
  methods: {
    nextPage() {
      if (this.page != 3) {
        this.page++;
        return;
      }
      this.$refs["submitButton"].disabled = true;
      this.createServer();
    },
    prevPage() {
      this.page--;
    },
    openFileWindow() {
      this.$refs["imageFileInput"].click();
    },
    updateImage(event) {
      const file = event.target.files[0];
      const fileName = file.name;
      this.$refs["imageFileButton"].innerHTML = fileName;
      const reader = new FileReader();
      reader.onload = (e) => {
        this.imageSrc = e.target.result;
      };
      reader.readAsDataURL(file);
    },
    imageURL() {
      this.imageSrc = this.$refs.imageURLInput.value;
    },
    imageURL() {
      this.imageSrc = this.$refs["imageURLInput"].value;
    },
    replaceWithSVG() {
      this.imageSrc = null;
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
    filteredVersions() {
      if (this.showSnapshots) {
        return this.minecraftVersions;
      }
      return this.minecraftVersions.filter(
        (version) => version.type === "release"
      );
    },
    async createServer() {
      this.creatingServer = true;
      let input = {
        name: this.serverName,
        serverType: this.serverType,
        mcVersion: this.serverVersion,
        image: this.imageSrc,
      };
      console.log(input);
      await invoke("add_server_command", input).then((response) => {
        console.log(response);
        if (response.startsWith("[success]")) {
          this.creatingServer = false;
          this.successfullyCreated = true;
        } else {
          this.creatingServer = false;
          this.failedToCreate = true;
          this.errorMessage = response;
        }
      });
    },
  },
};
</script>

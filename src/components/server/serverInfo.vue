<style lang="scss" scoped>
@import "../../assets/scss/global.scss";

$offline: #710b0b;
$online: #0e7a20;

h1 {
  margin: 2px 0;
  text-align: center;
  margin: 0 auto;
  max-width: 80%;
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
  margin: 0;
  display: flex;
  justify-content: center;
  gap: 10px;
  border-radius: $brr;
  background-color: $fg-a;

  padding: 10px;
  input,
  button {
    margin: 0;
    width: 200px;
    background-color: $fg-ma;
    transition: all 50ms;
    &:hover {
      background-color: $fg-m;
    }
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
      background-color: $fg-m;
      &::after {
        opacity: 1;
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

.delete-server {
  color: #cb0000;

  margin-top: 150px;

  h3 {
    margin: 0;
  }

  hr {
    border-color: #cb0000;
  }

  .delete-server-button {
    max-width: 300px;
    margin: 0 auto 0 0;

    background-color: #cb0000;
    transition: all 150ms;

    &:hover {
      background-color: #700000;
    }
  }
}

.save-button {
  background-image: url(/file-icons/save-icon.svg);
  background-repeat: no-repeat;
  background-position: 50% 50%;

  &.success {
    background-image: url(/file-icons/save-icon-success.svg);
    animation: 500ms successAnimation forwards ease-in-out;
  }
  &.fail {
    background-image: url(/file-icons/save-icon-fail.svg);
  }

  @keyframes successAnimation {
    0% {
      background-position: 50% 50%;
    }
    10% {
      background-position: 50% 80%;
    }
    80% {
      background-position: 50% 10%;
    }
    100% {
      background-position: 50% 50%;
    }
  }
}

.server-info-wrapper {
  max-width: 100%;
  overflow: hidden;

  .flex-heading {
    position: relative;
    background-color: $fg-a;
    padding: 5px;
    border-radius: 7px;

    &.online {
      background-color: $online;
    }
    &.offline {
      background-color: $offline;
    }

    .server-type {
      width: fit-content;
      display: flex;
      justify-content: space-between;
      gap: 5px;
      p {
        font-size: 1.3rem;
        margin: 2px;
        text-transform: capitalize;
      }
      .server-type-icon {
        width: 25px;
      }
    }
    h1 {
      position: absolute;
      left: 50%;
      transform: translateX(-50%);
      top: 0;
      margin: 0;
    }

    .simple-status {
      float: right;

      margin: 3px;
      padding: 2px;
      background-color: $fg-ma;
      border-radius: 5px;
    }
  }
  section {
    .name-input {
      display: flex;
      background-color: $fg-a;
      padding: 10px;
      width: 300px;
      border-radius: $brr;
      gap: 5px;

      input,
      button {
        margin: 0;
        background-color: $fg-ma;

        &:focus-visible,
        &:hover {
          background-color: $fg-m;
        }
      }

      input {
        flex-grow: 5;
        border-radius: 5px 3px 3px 5px;
      }
      button {
        flex-grow: 1;
        position: relative;
        width: 50px;
        padding: 10px;
        border-radius: 3px 5px 5px 3px;

        &.not-saved {
          background-color: $main-am;
        }
      }
    }
  }
}

.modal-delete {
  position: absolute;
  top: 0;
  width: 100%;
  height: 100%;
  background-color: $fg-ma;
  .modal-window {
    width: 400px;
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
    }

    button {
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

    .delete-button {
      background-color: #cb0000;
      &:hover {
        background-color: #710b0b;
      }
    }
  }
}
</style>

<template>
  <div class="server-info-wrapper">
    <div class="flex-heading" :class="status ? 'online' : 'offline'">
      <div class="simple-status">
        <div>
          {{ status ? "Online" : "Offline" }}
          <span v-if="status">: {{ players }}/{{ maxPlayers }}</span>
        </div>
      </div>
      <div class="server-type">
        <img
          class="server-type-icon"
          :src="'/file-icons/' + server.server_type + '.svg'"
          alt=""
        />
        <p>{{ server.server_type }} - {{ server.mc_version }}</p>
      </div>
      <h1>{{ server.name }}</h1>
    </div>
    <section>
      <h3>Nastavení</h3>
      <span>Server name:</span>
      <div class="name-input">
        <input type="text" v-model="server.name" maxlength="50" />
        <button
          @click="changeServerName"
          class="save-button"
          ref="nameSaveButton"
          :class="{ 'not-saved': originalServer.name != server.name }"
        ></button>
      </div>
      <span>Server image:</span>
      <div class="image-input">
        <div
          class="image-container"
          style="cursor: pointer"
          @click="openFileWindow"
        >
          <img
            :src="server.image"
            v-if="server.image"
            @error="replaceWithSVG"
            :key="server.image"
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
            Change server icon
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
            style="margin-top: 15px"
            ref="imageURLInput"
            placeholder="Image URL"
            @change="imageURL"
          />
        </div>
        <button
          @click="changeServerIcon"
          ref="saveImageButton"
          class="save-button"
          style="width: 80px"
        ></button>
      </div>
      <div class="delete-server">
        <h3>Danger zone:</h3>
        <hr />
        <button @click="showDelete = true" class="delete-server-button">
          Delete server
        </button>
      </div>
    </section>
    <div class="modal-delete" v-if="showDelete">
      <div class="modal-window">
        <h3>Are you sure you want to do this?</h3>
        <span>Do you want to delete your server with all of its content?</span>
        <ul>
          <li>World</li>
          <li>Mods</li>
          <li>PlayerData</li>
          <li>Config</li>
          <li>and everything on this server</li>
        </ul>
        <hr />
        <button @click="showDelete = false" class="cancel-button">
          Cancel
        </button>
        <button @click="remove_server_commamd" class="delete-button">DELETE</button>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
export default {
  name: "ServerInfo",
  props: {
    server_id: {
      default: "",
      type: String,
    },
    status: {
      default: false,
      type: Boolean,
    },
  },
  data() {
    return {
      server: {},
      originalServer: {},
      players: 0,
      maxPlayers: 0,
      showDelete: false,
    };
  },
  mounted() {
    this.getServerInfo();
  },
  methods: {
    async getServerInfo() {
      if (this.server_id != "") {
        this.server = JSON.parse(
          await invoke("get_server_info_command", { serverId: this.server_id })
        );
        this.originalServer = JSON.parse(JSON.stringify(this.server));
      }
    },
    async changeServerIcon() {
      if (
        (await invoke("edit_server", {
          serverId: this.server_id,
          field: "image",
          value: this.server.image,
        })) == null
      ) {
        this.$refs["saveImageButton"].classList.add("success");
        setTimeout(() => {
          this.$refs["saveImageButton"].classList.remove("success");
        }, 2000);
      } else {
        this.$refs["saveImageButton"].classList.add("fail");
        setTimeout(() => {
          this.$refs["saveImageButton"].classList.remove("fail");
        }, 2000);
      }
      this.getServerInfo();
    },
    async changeServerName() {
      if (
        (await invoke("edit_server", {
          serverId: this.server_id,
          field: "name",
          value: this.server.name,
        })) == null
      ) {
        this.$refs["nameSaveButton"].classList.add("success");
        setTimeout(() => {
          this.$refs["nameSaveButton"].classList.remove("success");
        }, 2000);
      } else {
        this.$refs["nameSaveButton"].classList.add("fail");
        setTimeout(() => {
          this.$refs["nameSaveButton"].classList.remove("fail");
        }, 2000);
      }
      this.getServerInfo();
    },
    async remove_server_commamd() {
      console.log( await invoke("remove_server_commamd",{serverId: this.server_id}));
      this.$router.push(`/`);
    },
    updateImage(event) {
      const file = event.target.files[0];
      const fileName = file.name;
      this.$refs["imageFileButton"].innerHTML = fileName;
      const reader = new FileReader();
      reader.onload = (e) => {
        this.server.image = e.target.result;
      };
      reader.readAsDataURL(file);
    },
    replaceImgbySvg() {
      this.server.image = null;
    },
    openFileWindow() {
      this.$refs["imageFileInput"].click();
    },
  },
};
</script>

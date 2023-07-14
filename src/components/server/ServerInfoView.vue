<style lang="scss" scoped>
@import "../../assets/scss/global.scss";

$offline: #710b0b;
$online: #0e7a20;
$starting: #a28814;

.image-input {
  width: fit-content;
  display: flex;
  justify-content: center;
  gap: 10px;
  padding: 10px;

  .inputs {
    display: flex;
    flex-flow: column nowrap;
    justify-content: space-between;
  }
}

.delete-server {
  margin-top: 150px;
  h3 {
    margin: 0;
    color: var(--color-red);
  }
  hr {
    border-color: var(--color-red);
  }
}

.server-info-wrapper {
  max-width: 100%;
  overflow: hidden;
  background-color: var(--color-raised-bg);
  border-radius: 7px;

  .flex-heading {
    position: relative;
    background-color: $fg-a;
    padding: 5px;
    border-radius: 7px;

    &.online {
      background-color: $online;
    }
    &.starting {
      background-color: $starting;
    }
    &.offline {
      background-color: $offline;
    }

    h1 {
      margin: 0;
    }

    .simple-status {
      float: right;

      margin: 8px;
      padding: 2px;
      background-color: $fg-ma;
      border-radius: 5px;
    }
  }
  .server-info-details {
    display: flex;
    flex-flow: row wrap;
    justify-content: space-around;
    padding: 10px;
    gap: var(--gap-xs);

    .motd {
      display: flex;
      gap: var(--gap-xs);
      flex-basis: fit-content;
      background-color: var(--color-button-bg);
      border-radius: var(--radius-sm);
      padding: var(--gap-xs);
      height: fit-content;
      color: #fff;
      & > span {
        margin: 5px 0;
      }
    }
    .players {
      background-color: var(--color-button-bg);
      border-radius: var(--radius-sm);
      padding: var(--gap-xs);
      min-width: 200px;
      h3 {
        margin: 2px 0;
      }
      .player-list {
        display: flex;
        flex-flow: column nowrap;
        span {
          display: flex;
          gap: var(--gap-xs);
          align-items: center;
        }
      }
    }
  }
}
.name-input {
  display: flex;
  padding: 10px;
  width: 320px;
  gap: 5px;
  box-sizing: border-box;
}

.modal-window {
  padding: 1rem;
  button {
    font-size: 1.2rem;
    width: 50%;
    display: inline-block;
  }
}

.advanced-settings {
  .field {
    label {
      display: block;
      font-size: 1.1rem;
      font-weight: 600;
      margin: 8px 0 0;
    }
    input[type="text"] {
      width: 100%;
    }
    input[type="number"] {
      width: 100px;
      margin-left: 20px;
    }
  }
}
.not-implemented {
  padding: 10px;
}
</style>

<template>
  <div class="server-info-wrapper">
    <div
      class="flex-heading"
      :class="
        status.online ? 'online' : status.starting ? 'starting' : 'offline'
      "
    >
      <div class="simple-status">
        <div>
          {{
            status.online
              ? "Online"
              : status.starting
              ? "Working..."
              : "Offline"
          }}
          <span v-if="status.online"
            >: {{ status.data.players.online }}/{{
              status.data.players.max
            }}</span
          >
        </div>
      </div>
      <h1>{{ server.name || "..." }}</h1>
    </div>
    <div class="server-info-details" v-if="status.online">
      <div class="motd">
        <Avatar class="motd-image" size="sm" :src="status.data.favicon" />
        <span v-html="serverMotd"></span>
      </div>
      <div
        class="players"
        v-if="
          status.data.players.sample && status.data.players.sample.length < 100
        "
      >
        <h3>Player list</h3>
        <div class="player-list">
          <span
            v-for="(player, index) in status.data.players.sample"
            :key="index + player"
          >
            <Avatar
              size="xs"
              :src="'https://mc-heads.net/avatar/' + player.id + '/64'"
            />
            {{ player.name }}
          </span>
        </div>
      </div>
    </div>
  </div>
  <section>
    <h3>Server name:</h3>
    <Card class="name-input">
      <input type="text" v-model="server.name" maxlength="50" />
      <SaveButton
        @click="changeServerName"
        style="height: 40px; min-width: 40px"
        :color="originalServer.name != server.name ? 'primary' : ''"
        ref="nameSaveButton"
      />
    </Card>
    <h3>Server image:</h3>
    <Card class="image-input">
      <Avatar size="md" :src="server.image"></Avatar>
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
            v-model="server.image"
            type="text"
            id="serverIcon"
            placeholder="Image url..."
          />
          <Button @click="() => (server.image = '')">
            <XIcon />
          </Button>
        </div>
      </div>
      <SaveButton
        @click="changeServerIcon"
        style="width: 80px; padding: 2.5rem 0; height: 95px"
        ref="saveImageButton"
        :color="originalServer.image != server.image ? 'primary' : ''"
      ></SaveButton>
    </Card>
    <h3>Server type & server version</h3>
    <Card>
      <ServerTypeIcon :type="server.s_type || 'Null'" />
      {{
        server.s_type
          ? server.s_type[0].toUpperCase() + server.s_type.slice(1)
          : "..."
      }}
      |
      {{ server.mc_version }}
      <Button
        outline
        style="margin-top: 10px"
        @click="$refs.changeServerTypeModal.show()"
        ><HashIcon />Change</Button
      >
    </Card>
    <h3>Advanced settings</h3>
    <Card class="advanced-settings">
      <UnknownIcon
        style="float: right; cursor: help"
        @mouseover="doNotMessWithAdvancedSetting"
      />
      <div class="field">
        <label for="javaPath">Java path</label>
        <input
          type="text"
          id="javaPath"
          name="java_path"
          v-model="server.java_path"
          placeholder="Custom java path..."
        />
      </div>
      <div class="field">
        <label for="memory"
          >Max memory ({{ server.memory_m || "??" }} mb)</label
        >
        <Slider
          id="memory"
          v-model="server.memory_m"
          :min="128"
          :max="16000"
          :step="1"
          unit="mb"
        />
      </div>
      <SaveButton
        style="width: 100%; margin-top: 10px"
        :color="
          server.memory_m != originalServer.memory_m ||
          server.java_path != originalServer.java_path
            ? 'primary'
            : ''
        "
        :disabled="
          !(
            server.memory_m != originalServer.memory_m ||
            server.java_path != originalServer.java_path
          )
        "
        @clicked="saveAdvancedSettings"
      ></SaveButton>
    </Card>

    <div class="delete-server">
      <h3>Danger zone:</h3>
      <hr />
      <Button @click="showDelete" outline color="danger">
        <TrashIcon /> Delete server
      </Button>
    </div>
  </section>
  <Modal header="Do you want to remove this server?" ref="deleteModal">
    <div class="modal-window">
      <span
        >This action will permanently delete your server, including all its
        content:</span
      >
      <ul>
        <li>World</li>
        <li>Mods</li>
        <li>Player data</li>
        <li>Configuration files</li>
        <li>and everything on this server</li>
      </ul>
      <hr />
      <Button
        @click="$refs.deleteModal.hide()"
        class="cancel-button"
        color="primary"
        large
        >Cancel</Button
      >
      <Button
        @click="remove_server_commamd"
        class="delete-button"
        color="danger"
        large
      >
        <TrashIcon />REMOVE
      </Button>
    </div>
  </Modal>
  <Modal ref="changeServerTypeModal" header="Feature Not Implemented">
    <div class="not-implemented">
      <p>Sorry, changing the server type is currently not implemented.</p>
      <p>
        If you know how to implement this feature, you can contribute it to this
        project.
      </p>
      <p>
        Adding this feature to the software would greatly enhance its
        capabilities.
      </p>
      <blockquote>
        If you have a critical need for this feature, here's a workaround: Open
        the server folder and manually change the server type.
      </blockquote>
      <Button
        @click="
          openLink(
            'https://github.com/oxydien/mc-server-manager/blob/main/src-tauri/src/server/create.rs#L17'
          )
        "
        ><LinkIcon />Github link</Button
      >
    </div>
  </Modal>
  <Notifications ref="notifsContainer" />
</template>

<script>
import {
  Avatar,
  Button,
  Modal,
  Badge,
  UploadIcon,
  Slider,
  XIcon,
  Card,
  FileInput,
  LinkIcon,
  TrashIcon,
  HashIcon,
  Notifications,
  UnknownIcon,
} from "omorphia";
import { invoke } from "@tauri-apps/api/tauri";
import SaveButton from "../SaveButton.vue";
import ServerTypeIcon from "../icons/ServerTypeIcon.vue";
export default {
  name: "ServerInfo",
  components: {
    ServerTypeIcon,
    Avatar,
    Button,
    Modal,
    Badge,
    UploadIcon,
    XIcon,
    Slider,
    Card,
    FileInput,
    LinkIcon,
    TrashIcon,
    HashIcon,
    SaveButton,
    UnknownIcon,
    Notifications,
  },
  props: {
    server_id: {
      default: "",
      type: String,
    },
    status: {
      default: false,
      type: Object,
    },
  },
  data() {
    return {
      server: {},
      config: {},
      originalServer: {},
      players: 0,
      maxPlayers: 0,

      imagePrompt: "Change image",
      messWithAdvancedSetting: false,
    };
  },
  mounted() {
    this.getServerInfo().then(() => {
      this.getConfig();
    });
  },
  methods: {
    async getServerInfo() {
      if (this.server_id != "") {
        this.server = JSON.parse(
          await invoke("get_server_info_command", { serverId: this.server_id })
        );
        this.server.memory_m = parseInt(this.server.memory_m);
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
        this.$refs["saveImageButton"].success = true;
        setTimeout(() => {
          this.$refs["saveImageButton"].success = false;
        }, 2000);
      } else {
        this.$refs["saveImageButton"].color = "danger";
        setTimeout(() => {
          this.$refs["saveImageButton"].color = "danger";
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
        this.$refs["nameSaveButton"].success = true;
        setTimeout(() => {
          this.$refs["nameSaveButton"].success = false;
        }, 2000);
      } else {
        this.$refs["nameSaveButton"].color = "danger";
        setTimeout(() => {
          this.$refs["nameSaveButton"].color = "danger";
        }, 2000);
      }
      this.getServerInfo();
    },
    async getConfig() {
      const response = JSON.parse(await invoke("get_config_command"));
      this.config = response;
      if (!this.server.java_path) {
        this.server.java_path = response[this.jdkVersion];
        this.originalServer.java_path = response[this.jdkVersion];
      }
      if (!this.server.memory_m) {
        this.server.memory_m = parseInt(response.memory_m);
        this.originalServer.memory_m = parseInt(response.memory_m);
      }
    },
    openLink(link) {
      window.open(link, "_blank");
    },
    showDelete() {
      this.$refs.deleteModal.show();
    },
    async remove_server_commamd() {
      console.log(
        await invoke("remove_server_commamd", { serverId: this.server_id })
      );
      this.$router.push(`/`);
    },
    updateImage(event) {
      const file = event[0];
      const fileName = file.name;
      const shortenedName = this.shortenFileName(fileName, 18);

      this.imagePrompt = shortenedName;

      const reader = new FileReader();
      reader.onload = (e) => {
        this.server.image = e.target.result;
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
    doNotMessWithAdvancedSetting() {
      if (!this.messWithAdvancedSetting) {
        this.messWithAdvancedSetting = true;
        this.$refs.notifsContainer.addNotification({
          title: "Don't mess with it",
          text: "if you don't know what it does",
          type: "warn",
        });
      }
    },
    async saveAdvancedSettings() {
      console.log(
        "save java_path:",
        await invoke("edit_server", {
          serverId: this.server_id,
          field: "java_path",
          value: this.server.java_path,
        })
      );
      console.log(
        "save memory_m:",
        await invoke("edit_server", {
          serverId: this.server_id,
          field: "memory_m",
          value: "" + this.server.memory_m,
        })
      );
      this.getServerInfo().then(() => {
        this.getConfig();
      });
    },
  },
  computed: {
    jdkVersion() {
      if (
        this.server.mc_version > "1.16.5" ||
        this.server.mc_version >= "20w45a"
      ) {
        return "jdk17";
      }
      return "jdk8";
    },
    serverMotd() {
      let codeSplit = this.status.data.description.text.split(
        /([§][0-9a-fA-FfklmnorxFKLMNORX])/g
      );

      let colorHex = "";
      let fontStyle = "";
      let skipNext = 0;
      let resultHTML = "";
      let colorCodeToHex = {
        "§0": "#000000",
        "§1": "#0000AA",
        "§2": "#00AA00",
        "§3": "#00AAAA",
        "§4": "#AA0000",
        "§5": "#AA00AA",
        "§6": "#FFAA00",
        "§7": "#AAAAAA",
        "§8": "#555555",
        "§9": "#5555FF",
        "§a": "#55FF55",
        "§b": "#55FFFF",
        "§c": "#FF5555",
        "§d": "#FF55FF",
        "§e": "#FFFF55",
        "§f": "#FFFFFF",
      };
      let extras = {
        "§k": "obfuscated;",
        "§l": "font-weight: bold;",
        "§m": "text-decoration: line-through;",
        "§n": "text-decoration:underline;",
        "§o": "font-style: oblique 20deg;",
      };

      codeSplit.forEach((item, index) => {
        if (skipNext > 0) {
          skipNext--;
          return;
        }
        const stringToLowerCase = item.toLowerCase();
        if (stringToLowerCase === "§r") {
          colorHex = "";
          fontStyle = "";
        } else if (colorCodeToHex.hasOwnProperty(stringToLowerCase)) {
          colorHex = colorCodeToHex[stringToLowerCase];
        } else if (extras.hasOwnProperty(stringToLowerCase)) {
          fontStyle = extras[stringToLowerCase];
        } else if (stringToLowerCase === "§x") {
          const hexColourCode = (
            "#" +
            codeSplit[index + 2] +
            codeSplit[index + 4] +
            codeSplit[index + 6] +
            codeSplit[index + 8] +
            codeSplit[index + 10] +
            codeSplit[index + 12]
          ).replaceAll("§", "");
          resultHTML += `<span style="color:${hexColourCode}${fontStyle}">${
            codeSplit[index + 13]
          }</span>`;
          skipNext = 13;
        } else {
          let resultColor = "";
          if (colorHex !== "") {
            resultColor = `color:${colorHex};`;
          }
          if (item !== "") {
            let textContent = item
              .replace(/ /g, "\u00a0")
              .replace(/&/g, "&amp;")
              .replace(/</g, "&lt;")
              .replace(/>/g, "&gt;")
              .replace(/\"/g, "&quot;")
              .replace(/\'/g, "&#39;")
              .replace(/\n/g, "<br/>");
            if (resultColor.length !== 0 || fontStyle.length !== 0) {
              resultHTML += `<span style="${resultColor}${fontStyle}">${textContent}</span>`;
            } else {
              resultHTML += textContent;
            }
          }
        }
      });

      return resultHTML;
    },
  },
};
</script>

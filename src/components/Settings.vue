<style lang="scss" scoped>
@import "../assets/scss/global.scss";

.save-settings {
  float: right;
}
h1 {
  margin: 0 auto;
  color: $main-h;
}
.form {
  background-color: $fg-ma;
  padding: 5px;
  border-radius: $brr;
  box-sizing: border-box;
  font-size: 1.3rem;
  h2 {
    margin-top: 0;
    margin-bottom: 10px;
  }

  .input {
    margin: 5px;
    background-color: $fg-ma;
    box-sizing: border-box;
    border-radius: 5px;

    label {
      display: block;
      margin: 8px 0 5px 0;
    }
    .iconified-input {
      width: 100%;
    }
    input {
      padding-left: 10px;
    }
  }
}
</style>

<template>
  <main>
    <Button
      color="primary"
      class="save-settings"
      :disabled="!hasChanges"
      @click="saveConfig"
    >
      <SaveIcon />
      Save
    </Button>
    <h1>Settings</h1>
    <hr />
    <div class="form" :key="sortedSettingsJson">
      <div
        class="input"
        v-for="(setting, key) in sortedSettingsJson"
        :key="key"
      >
        <label :for="key">{{ settingsDictionary[key] }}:{{ modifiedSettingsJson[key] }}</label>
        <div class="iconified-input">
          <input
            v-model="modifiedSettingsJson[key]"
            @input="checkIfChanged"
            type="text"
          />
          <Button @click="() => (modifiedSettingsJson[key] = '')">
            <XIcon />
          </Button>
        </div>
      </div>
    </div>
    <Notifications ref="notifsContainer" />
  </main>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import { Button, SaveIcon, XIcon, Notifications } from "omorphia";

export default {
  name: "SettingsPage",
  data() {
    return {
      settingsJson: {},
      modifiedSettingsJson: {},
      hasChanges: false,
      settingsDictionary: {
        arguments: "Java arguments",
        jdk17: "Java 17 location (java.exe)",
        jdk8: "Java 8 location (java.exe)",
        memory_m: "Java max memory in MB (Xmx)",
      },
    };
  },
  components: {
    Button,
    SaveIcon,
    Notifications,
    XIcon,
  },
  mounted() {
    this.getSettings();
  },
  computed: {
    sortedSettingsJson() {
      const sortedKeys = [
        "jdk17",
        "jdk8",
        ...Object.keys(this.settingsJson).filter(
          (key) => !["jdk17", "jdk8"].includes(key)
        ),
      ];
      return Object.fromEntries(
        sortedKeys.map((key) => [key, this.settingsJson[key]])
      );
    },
  },
  methods: {
    async getSettings() {
      const response = JSON.parse(await invoke("get_config_command"));
      this.$refs.notifsContainer.addNotification({
        title: "Get settings",
        text: JSON.stringify(response),
        type: "info",
      });
      this.settingsJson = response;
      this.modifiedSettingsJson = JSON.parse(JSON.stringify(response));
    },
    async saveConfig() {
      for (const key in this.modifiedSettingsJson) {
        if (
          Object.prototype.hasOwnProperty.call(this.modifiedSettingsJson, key)
        ) {
          const value = this.modifiedSettingsJson[key];
          let response = await invoke("set_config_command", {
            name: key,
            value,
          });
          if (response != null) {
            this.$refs.notifsContainer.addNotification({
              title: "Set settings " + key,
              text: JSON.stringify(response),
              type: "error",
            });
          }
        }
      }
      this.hasChanges = false;
    },
    checkIfChanged() {
      this.hasChanges = Object.keys(this.modifiedSettingsJson).some(
        (key) => this.modifiedSettingsJson[key] !== this.settingsJson[key]
      );
    },
  },
};
</script>

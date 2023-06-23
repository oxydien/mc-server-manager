<style lang="scss" scoped>
@import "../assets/scss/global.scss";

.save-settings {
  float: right;
  background-color: $main;
  border: none;
  font-size: 1.3rem;
  color: $txt-h;
  border-radius: 5px;
  padding: 0.3rem 0.5rem;
  cursor: pointer;

  svg {
    float: left;
    margin-right: 2px;
  }

  &:disabled {
    background-color: $main-m;
    color: $txt-m;
    cursor: not-allowed;
  }
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

    input {
      display: block;
      width: 100%;
      font-size: 1.3rem;
      color: $txt-m;
      border: none;
      padding: 0.375rem 0.532rem;
      border-radius: 5px;
      background-color: $fg-a;
      box-sizing: border-box;

      &:focus {
        color: $txt-h;
      }
    }
  }
}
</style>

<template>
  <main>
    <button class="save-settings" :disabled="!hasChanges" @click="saveConfig">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
      >
        <path
          fill="none"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M8 4H6a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.828a2 2 0 0 0-.586-1.414l-1.828-1.828A2 2 0 0 0 16.172 4H15M8 4v4a1 1 0 0 0 1 1h5a1 1 0 0 0 1-1V4M8 4h7M7 17v-3a1 1 0 0 1 1-1h8a1 1 0 0 1 1 1v3"
        /></svg>
      Save
    </button>
    <h1>Settings</h1>
    <hr />
    <div class="form" :key="sortedSettingsJson">
      <div
        class="input"
        v-for="(setting, key) in sortedSettingsJson"
        :key="key"
      >
        <label :for="key">{{ settingsDictionary[key] }}:</label>
        <input
          type="text"
          :id="key"
          :placeholder="'Enter ' + key + '..'"
          v-model="modifiedSettingsJson[key]"
          @input="checkIfChanged"
        />
      </div>
    </div>
  </main>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";

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
      console.log(response);
      this.settingsJson = response;
      this.modifiedSettingsJson = JSON.parse(JSON.stringify(response));
    },
    async saveConfig() {
      for (const key in this.modifiedSettingsJson) {
        if (
          Object.prototype.hasOwnProperty.call(this.modifiedSettingsJson, key)
        ) {
          const value = this.modifiedSettingsJson[key].value;
          await invoke("set_config_command", { name: key, value });
        }
      }
      this.hasChanges = false;
    },
    checkIfChanged() {
      this.hasChanges = Object.keys(this.modifiedSettingsJson).some(
        (key) =>
          this.modifiedSettingsJson[key] !== this.settingsJson[key]
      );
    },
  },
};
</script>

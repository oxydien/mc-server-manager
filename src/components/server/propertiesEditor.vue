<style lang="scss" scoped>
@import "../../assets/scss/global.scss";

h2 {
  padding: 6px 10px;
  position: sticky;
  top: -8px;
  background-color: $fg-h;
  margin: 5px 0 0 0;
  border-radius: 3px 3px 5px 5px;

  display: flex;
  justify-content: space-between;

  button {
    display: flex;
    align-items: center;
    font-family: Arial, Helvetica, sans-serif;
    font-size: 1.1rem;
    margin: -5px 0;
    overflow: hidden;

    color: $txt;
    background-color: $main-m;
    padding: 0 0.5rem;
    border: none;
    border-radius: 6px;
    box-sizing: border-box;
    cursor: not-allowed;

    &.allowed {
      cursor: pointer;
      background-color: $main;
    }
  }
}
.form-group {
  padding: 0.5rem;
  display: flex;

  span {
    padding: 0.6rem 0;
    margin: 0 2px;
  }

  label {
    width: 40ch;
    padding: 0.6rem 0;
    font-size: 1.2rem;
  }

  input {
    width: 100%;
    font-family: Arial, Helvetica, sans-serif;
    font-size: 1.1rem;
    display: block;
    overflow: hidden;

    color: $txt;
    background-color: $fg-a;
    padding: 0.505rem 0.808rem;
    border: none;
    border-radius: 6px;
    box-sizing: border-box;
  }
}

.save-button {
  background-image: url(/file-icons/save-icon.svg);
  background-repeat: no-repeat;
  background-position: 50% 50%;
  width: 50px;

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
</style>

<template>
  <section>
    <h2>
      Property editor
      <button
        @click="saveServerProperties"
        class="save-button"
        :class="
          JSON.stringify(originalPropertyFile) != JSON.stringify(propertyFile)
            ? 'allowed'
            : ''
        "
      >

      </button>
    </h2>
    <div v-for="(value, key) in propertyFile" :key="key" class="form-group">
      <label :for="key">{{ key }}</label>
      <span>:</span>
      <input
        :id="key"
        v-model="propertyFile[key]"
        class="form-control"
        type="text"
      />
    </div>
  </section>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
export default {
  name: "PropertiesEditor",
  props: {
    server_id: {
      default: "",
      type: String,
    },
  },
  data() {
    return {
      propertyFile: {},
      originalPropertyFile: {},
    };
  },
  mounted() {
    this.loadServerProperties();
  },
  methods: {
    async loadServerProperties() {
      if (this.server_id != "") {
        this.propertyFile = JSON.parse(
          await invoke("read_server_properties", { serverId: this.server_id })
        );
        this.originalPropertyFile = JSON.parse(
          JSON.stringify(this.propertyFile)
        );
        console.log(this.propertyFile);
      }
    },
    async saveServerProperties() {
      console.log(
        await invoke("write_server_properties", {
          serverId: this.server_id,
          jsonString: JSON.stringify(this.propertyFile),
        }).then(() => {
          this.loadServerProperties();
        })
      );
    },
  },
};
</script>

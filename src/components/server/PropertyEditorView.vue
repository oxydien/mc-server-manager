<style lang="scss" scoped>
@import "../../assets/scss/global.scss";

h2 {
  padding: 6px 10px;
  position: sticky;
  top: -8px;
  background-color: var(--color-raised-bg);
  margin: 5px 0 0 0;
  border-radius: 3px 3px 5px 5px;

  display: flex;
  justify-content: space-between;
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
  }
}
</style>

<template>
  <section>
    <h2>
      Property editor
      <SaveButton color="highlight" :disabled="!(JSON.stringify(originalPropertyFile) != JSON.stringify(propertyFile))"/>
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
    <InfoTable v-if="propertyFile === ''" type="no_property_file" />
  </section>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import SaveButton from "../SaveButton.vue"
import InfoTable from "../icons/InfoTable.vue"
export default {
  name: "PropertiesEditor",
  components: {
    SaveButton,
    InfoTable
  },
  props: {
    server_id: {
      default: "",
      type: String,
    },
  },
  data() {
    return {
      propertyFile: '',
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

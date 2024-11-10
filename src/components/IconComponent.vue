<template>
  <div class="icon-container" @click="handleClick">
    <img :src="iconBase64" alt="Icon" class="icon" />
    <div class="file-name">{{ lnkName }}</div>
  </div>
</template>

<script>
import { open } from "@tauri-apps/plugin-shell";
export default {
  name: "IconComponent",
  props: {
    iconBase64: {
      type: String,
      required: true,
    },
    lnkName: {
      type: String,
      required: true,
    },
    path: {
      type: String,
      required: true,
    },
  },
  methods: {
    handleClick() {
      open(this.path);
    },
  },
};
</script>

<style scoped>
.icon-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 64px;
  height: 96px; /* Adjust height to accommodate file name */
  box-sizing: border-box;
}

.icon {
  width: 64px;
  height: 64px;
  object-fit: contain;
  margin-bottom: 4px; /* Space between icon and file name */
}

.file-name {
  font-size: 12px;
  text-align: center;
  word-wrap: break-word;
}
</style>

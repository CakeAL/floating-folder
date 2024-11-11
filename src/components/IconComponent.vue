<template>
  <div class="icon-container" @click="handleClick">
    <img :src="iconBase64" alt="Icon" class="icon" />
    <div class="file-name">{{ name }}</div>
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
    name: {
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
  display: inline-flex;
  flex-direction: column;
  align-items: center;
  width: 64px;
  height: 64px; 
  box-sizing: border-box;
  padding-top: 10px;
}

.icon {
  width: 32px;
  height: 32px;
  object-fit: contain;
  margin-bottom: 2px; 
}

.file-name {
  font-size: 12px;
  width: 64px;
  height: 20px;
  text-align: center;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
</style>

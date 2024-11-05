<script setup lang="ts">
import { ref } from "vue";

const isExpended = ref(false);

const openFolder = () => {
  isExpended.value = true;
};

const closeFolder = () => {
  isExpended.value = false;
};
</script>

<template>
  <div class="container">
    <div
      class="folder"
      :class="{ expanded: isExpended }"
      data-tauri-drag-region
      @click="openFolder"
      @mouseleave="closeFolder"
    ></div>
  </div>
</template>

<style scoped>
.container {
  width: 192px;
  height: 192px;
  position: relative;
}

.folder {
  width: 64px;
  height: 64px;
  background-color: rgba(200, 200, 200, 0.3);
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  border-radius: 16px;
  transition: all 0.3s ease-in-out;
  cursor: pointer;
}

.folder.expanded {
  width: 192px;
  height: 192px;
  transform: translate(0, 0);
  top: 0px;
  left: 0px;
  border-radius: 16px;
}
</style>

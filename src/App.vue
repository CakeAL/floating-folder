<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, ref } from "vue";
import { Icon } from "./entity";
import IconComponent from "./components/IconComponent.vue";

// const isExpended = ref(false);

// const openFolder = () => {
//   isExpended.value = true;
// };

// const closeFolder = () => {
//   isExpended.value = false;
// };

// 禁止刷新
const label = ref<any>("");

onMounted(() => {
  if ((window as any).label) {
    label.value = (window as any).label;
  }
  // 获取图标
  getIcons();
});

// move
const appWindow = getCurrentWindow();

let moveTimeout: number | undefined;

appWindow.listen("tauri://move", () => {
  clearTimeout(moveTimeout);
  moveTimeout = setTimeout(async () => {
    let pos = await appWindow.outerPosition();
    const scaleFactor = await appWindow.scaleFactor();
    const logicalX = parseInt((pos.x / scaleFactor).toFixed(0));
    const logicalY = parseInt((pos.y / scaleFactor).toFixed(0));
    await invoke("moved_folder", {
      label: label.value,
      x: logicalX,
      y: logicalY,
    });
  }, 100);
});

// drag

appWindow.listen("tauri://drag-drop", async (event) => {
  console.log(event);
  await invoke("send_path_to_folder", {
    label: label.value,
    path: (event.payload as any).paths,
  }).then(() => getIcons());
});

// Icon
const icons = ref<Array<Icon>>([]);
const getIcons = async () => {
  let res = await invoke("get_icons", { label: label.value }).catch((err) =>
    console.log(err)
  );
  icons.value = JSON.parse(res as unknown as string);
};
</script>

<template>
  <div class="container">
    <!-- :class="{ expanded: isExpended }" -->
    <div class="folder" data-tauri-drag-region>
      Hi, I'm {{ label }}
      <IconComponent
        v-for="(icon, index) in icons"
        :key="index"
        :iconBase64="icon.base64"
        :lnkName="icon.lnkName"
        :path="icon.path"
      ></IconComponent>
    </div>
    <!--       @click="openFolder"
      @mouseleave="closeFolder" -->
  </div>
</template>

<style scoped>
.container {
  width: 192px;
  height: 192px;
  position: relative;
  pointer-events: none;
}

.folder {
  width: 192px;
  height: 192px;
  background-color: rgba(200, 200, 200, 0.3);
  position: absolute;
  /* top: 64px;
  left: 64px; */
  border-radius: 16px;
  /* transition: all 0.3s ease-in-out; */
  cursor: pointer;
  pointer-events: all;
}

/* .folder:hover {
  width: 192px;
  height: 192px;
  top: 0px;
  left: 0px;
  border-radius: 16px;
  pointer-events: all;
} */
</style>

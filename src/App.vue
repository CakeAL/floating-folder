<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, ref } from "vue";
import { Icon } from "./entity";
import { Menu } from "@tauri-apps/api/menu";
import IconComponent from "./components/IconComponent.vue";
import Thumbnail from "./components/Thumbnail.vue";

onMounted(() => {
  // 获取图标
  getIcons();
});

// move & label
const appWindow = getCurrentWindow();
const label = appWindow.label;

let moveTimeout: number | undefined;

appWindow.listen("tauri://move", () => {
  clearTimeout(moveTimeout);
  moveTimeout = setTimeout(async () => {
    let pos = await appWindow.outerPosition();
    const scaleFactor = await appWindow.scaleFactor();
    const logicalX = parseInt((pos.x / scaleFactor).toFixed(0));
    const logicalY = parseInt((pos.y / scaleFactor).toFixed(0));
    await invoke("moved_folder", {
      label,
      x: logicalX,
      y: logicalY,
    });
  }, 100);
});

// drag
appWindow.listen("tauri://drag-drop", async (event) => {
  // console.log(event);
  await invoke("send_path_to_folder", {
    label: label,
    path: (event.payload as any).paths,
  }).then(() => getIcons());
});

// Icon
const icons = ref<Array<Icon>>([]);
const getIcons = async () => {
  let res = await invoke("get_icons", { label }).catch((err) =>
    console.log(err)
  );
  icons.value = JSON.parse(res as unknown as string);
};

// Context Menu
const menuPromise = Menu.new({
  items: [
    {
      id: "open_folder",
      text: "打开文件夹所在位置",
      action: async () => {
        await invoke("open_folder", { label });
      },
    },
    {
      id: "del_folder",
      text: "删除该文件夹",
      action: async () => {
        await invoke("del_folder", { label });
      },
    },
    {
      id: "reflesh",
      text: "刷新",
      action: () => location.reload(),
    },
  ],
});

const handleClick = async (event: { preventDefault: () => void }) => {
  event.preventDefault();
  const menu = await menuPromise;
  menu.popup();
};

// if close and scale
const if_close = ref(true);

const handleMouseEnter = () => {
  invoke("scale_folder", { label, len: 192.0 });
  if_close.value = false;
};

const handleMouseLeave = () => {
  setTimeout(() => {
    if_close.value = true;
    invoke("scale_folder", { label, len: 64.0 });
  }, 200);
};
</script>

<template>
  <div
    @contextmenu="handleClick"
    class="folder"
    v-on:mouseenter="handleMouseEnter"
    v-on:mouseleave="handleMouseLeave"
  >
    <div v-if="if_close" style="width: 64px; height: 64px">
      <Thumbnail
        v-for="(icon, index) in icons"
        :key="index"
        :iconBase64="icon.base64"
      />
    </div>
    <div v-else style="width: 192px; height: 192px" data-tauri-drag-region>
      <IconComponent
        v-for="(icon, index) in icons"
        :key="index"
        :iconBase64="icon.base64"
        :name="icon.name"
        :path="icon.path"
      />
    </div>
  </div>
</template>

<style scoped>
.folder {
  overflow: hidden;
  width: 64px;
  height: 64px;
  background-color: rgba(200, 200, 200, 0.3);
  position: absolute;
  top: 50%;
  left: 50%;
  border-radius: 10px;
  transition: all 0.2s ease-in-out;
  transform: translate(-50%, -50%);
  cursor: pointer;
  pointer-events: all;
  scrollbar-width: 0px;
}

.folder:hover {
  width: 192px;
  height: 192px;
  pointer-events: all;
}
</style>

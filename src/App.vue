<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, ref } from "vue";
import { Icon } from "./entity";
import { Menu } from "@tauri-apps/api/menu";
import IconComponent from "./components/IconComponent.vue";

// const isExpended = ref(false);

// const openFolder = () => {
//   isExpended.value = true;
// };

// const closeFolder = () => {
//   isExpended.value = false;
// };

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
  console.log(event);
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
  ],
});

const handleClick = async (event: { preventDefault: () => void }) => {
  event.preventDefault();
  const menu = await menuPromise;
  menu.popup();
};
</script>

<template>
  <div class="container" @contextmenu="handleClick">
    <!-- :class="{ expanded: isExpended }" -->
    <div class="folder" data-tauri-drag-region>
      <!-- Hi, I'm {{ label }} -->
      <IconComponent
        v-for="(icon, index) in icons"
        :key="index"
        :iconBase64="icon.base64"
        :name="icon.name"
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
  overflow-x: hidden;
  overflow-y: hidden;
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

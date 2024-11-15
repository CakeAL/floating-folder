<template>
  <div class="icon-container">
    <img :src="iconBase64" alt="Icon" class="icon" />
  </div>
</template>

<script>
export default {
  name: "Thumbnail",
  props: {
    iconBase64: {
      type: String,
      required: true,
    },
  }
};
</script>

<style scoped>
.icon-container {
  display: inline-flex;
  flex-direction: column;
  align-items: center;
  width: calc(100vw / 3);
  height: calc(100vh / 3);
  box-sizing: border-box;
  pointer-events: none;
  user-select: none;
  transition: all 0.2s ease-in-out;
}

.icon {
  width: calc(100vw / 4);
  height: calc(100vh / 4);
  object-fit: contain;
  pointer-events: all;
}
</style>

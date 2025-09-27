<template>
  <q-layout view="lHh Lpr lFf">
  <q-header class="header no-shadow">
    <q-toolbar class="row items-center justify-around">
      <!-- Left Section -->
      <div
        class="col-auto"
        style="display: flex; gap: 8px; align-items: center; margin-right: 16px;"
        ref="navDiv"
      >
        <q-btn round size="4px" style="background:#ff5f57; box-shadow:0 0 1px #333;" flat />
        <q-btn round size="4px" style="background:#febc2e; box-shadow:0 0 1px #333;" flat />
        <q-btn round size="4px" style="background:#28c940; box-shadow:0 0 1px #333;" flat />
      </div>

      <!-- Centered Title -->
      <q-toolbar-title
        class="col text-center"
        style="padding-left: 0px; padding-right: 0px;"
      >
        Image converter
      </q-toolbar-title>

      <div class="col-auto" :style="`width: ${navDivWidth}px`" style="margin-left: 16px;"></div>
    </q-toolbar>
  </q-header>

    <q-page-container style="background-color: var(--background-color);" >
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'MainLayout',
  setup() {
    const navDiv = ref<HTMLElement | null>(null);
    const navDivWidth = computed<number>(() => {
      if (navDiv.value) {
        return navDiv.value.clientWidth;
      }
      return 0;
    });
    return {
      navDiv,
      navDivWidth
    };
  },
});
</script>

<style lang="scss" scoped>
.header {
  background-color: var(--header-color-bg);
  border-bottom: 1px solid rgb(229, 231, 235);
  display: flex;
  align-items: center;
  color: #030213;
}
</style>
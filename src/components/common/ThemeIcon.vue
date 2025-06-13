<script setup lang="ts">
import { computed } from 'vue'
import moonIcon from '../../assets/svg/moon-rising-twotone-alt-loop.svg?raw'
import sunIcon from '../../assets/svg/moon-to-sunny-outline-loop-transition.svg?raw'

interface Props {
  theme: string
}

const props = defineProps<Props>()

const iconSvg = computed(() => {
  return props.theme === 'light' ? sunIcon : moonIcon
})
</script>

<template>
  <div
    class="theme-icon"
    v-html="iconSvg"
  />
</template>

<style scoped>
.theme-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 1em;
  height: 1em;
}

.theme-icon :deep(svg) {
  width: 100%;
  height: 100%;
}
</style>

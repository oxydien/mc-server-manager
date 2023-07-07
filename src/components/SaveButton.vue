<style lang="scss" scoped>
:where(button) {
  margin-top: 0;
  color: var(--color-base);
}

.success svg { 
  animation: 500ms successAnimation forwards ease-in-out;
}

@keyframes successAnimation {
    0% {
      transform: translate( 50% 50%);
    }
    10% {
      transform: translate( 50% 80%);
    }
    80% {
      transform: translate( 50% 10%);
    }
    100% {
      transform: translate( 50% 50%);
    }
  }
</style>

<template>
  <button
    class="btn icon-only"
    :class="{
      'success': props.success,
      'btn-large': props.large,
      'btn-raised': props.color === 'raised',
      'btn-danger': props.color === 'danger',
      'btn-primary': props.color === 'primary',
      'btn-secondary': props.color === 'secondary',
      'btn-highlight': props.color === 'highlight',
      'color-accent-contrast': accentedButton,
    }"
    @click="$emit('clicked')"
  >
    <SaveIcon />
  </button>
</template>

<script setup>
import { SaveIcon } from "omorphia"
import { computed } from 'vue'

const props = defineProps({
  color: {
    type: String,
    default: 'default',
  },
  success: {
    type: Boolean,
    default: false,
  },
  large: {
    type: Boolean,
    default: false,
  },
})

defineEmits(['clicked'])

const accentedButton = computed(() => ['danger', 'primary'].includes(props.color))
</script>
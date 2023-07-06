<template>
  <svg
    v-if="type.toLowerCase() === 'fabric'"
    xmlns="http://www.w3.org/2000/svg"
    xml:space="preserve"
    fill-rule="evenodd"
    stroke-linecap="round"
    stroke-linejoin="round"
    clip-rule="evenodd"
    viewBox="0 0 24 24"
  >
    \n
    <path fill="none" d="M0 0h24v24H0z" />
    \n
    <path
      fill="none"
      stroke="currentColor"
      stroke-width="23"
      d="m820 761-85.6-87.6c-4.6-4.7-10.4-9.6-25.9 1-19.9 13.6-8.4 21.9-5.2 25.4 8.2 9 84.1 89 97.2 104 2.5 2.8-20.3-22.5-6.5-39.7 5.4-7 18-12 26-3 6.5 7.3 10.7 18-3.4 29.7-24.7 20.4-102 82.4-127 103-12.5 10.3-28.5 2.3-35.8-6-7.5-8.9-30.6-34.6-51.3-58.2-5.5-6.3-4.1-19.6 2.3-25 35-30.3 91.9-73.8 111.9-90.8"
      transform="matrix(.08671 0 0 .0867 -49.8 -56)"
    />
    \n
  </svg>
  <svg
    v-else-if="type.toLowerCase() === 'quilt'"
    xmlns:xlink="http://www.w3.org/1999/xlink"
    xml:space="preserve"
    fill-rule="evenodd"
    stroke-linecap="round"
    stroke-linejoin="round"
    stroke-miterlimit="2"
    clip-rule="evenodd"
    viewBox="0 0 24 24"
  >
    <defs>
      <path
        id="quilt"
        fill="none"
        stroke="currentColor"
        stroke-width="65.6"
        d="M442.5 233.9c0-6.4-5.2-11.6-11.6-11.6h-197c-6.4 0-11.6 5.2-11.6 11.6v197c0 6.4 5.2 11.6 11.6 11.6h197c6.4 0 11.6-5.2 11.6-11.7v-197Z"
      ></path>
    </defs>
    <path fill="none" d="M0 0h24v24H0z"></path>
    <use
      xlink:href="#quilt"
      stroke-width="65.6"
      transform="matrix(.03053 0 0 .03046 -3.2 -3.2)"
    ></use>
    <use
      xlink:href="#quilt"
      stroke-width="65.6"
      transform="matrix(.03053 0 0 .03046 -3.2 7)"
    ></use>
    <use
      xlink:href="#quilt"
      stroke-width="65.6"
      transform="matrix(.03053 0 0 .03046 6.9 -3.2)"
    ></use>
    <path
      fill="none"
      stroke="currentColor"
      stroke-width="70.4"
      d="M442.5 234.8c0-7-5.6-12.5-12.5-12.5H234.7c-6.8 0-12.4 5.6-12.4 12.5V430c0 6.9 5.6 12.5 12.4 12.5H430c6.9 0 12.5-5.6 12.5-12.5V234.8Z"
      transform="rotate(45 3.5 24) scale(.02843 .02835)"
    ></path>
  </svg>
  <svg viewBox="0 0 20 20" fill="currentColor" v-else-if="type.toLowerCase() === 'vanilla'">
    <path
      fill-rule="evenodd"
      d="M9.504 1.132a1 1 0 01.992 0l1.75 1a1 1 0 11-.992 1.736L10 3.152l-1.254.716a1 1 0 11-.992-1.736l1.75-1zM5.618 4.504a1 1 0 01-.372 1.364L5.016 6l.23.132a1 1 0 11-.992 1.736L4 7.723V8a1 1 0 01-2 0V6a.996.996 0 01.52-.878l1.734-.99a1 1 0 011.364.372zm8.764 0a1 1 0 011.364-.372l1.733.99A1.002 1.002 0 0118 6v2a1 1 0 11-2 0v-.277l-.254.145a1 1 0 11-.992-1.736l.23-.132-.23-.132a1 1 0 01-.372-1.364zm-7 4a1 1 0 011.364-.372L10 8.848l1.254-.716a1 1 0 11.992 1.736L11 10.58V12a1 1 0 11-2 0v-1.42l-1.246-.712a1 1 0 01-.372-1.364zM3 11a1 1 0 011 1v1.42l1.246.712a1 1 0 11-.992 1.736l-1.75-1A1 1 0 012 14v-2a1 1 0 011-1zm14 0a1 1 0 011 1v2a1 1 0 01-.504.868l-1.75 1a1 1 0 11-.992-1.736L16 13.42V12a1 1 0 011-1zm-9.618 5.504a1 1 0 011.364-.372l.254.145V16a1 1 0 112 0v.277l.254-.145a1 1 0 11.992 1.736l-1.735.992a.995.995 0 01-1.022 0l-1.735-.992a1 1 0 01-.372-1.364z"
      clip-rule="evenodd"
    ></path>
  </svg>
</template>

<script setup>
defineProps({
  type: {
    required: true,
    type: String,
  },
});
</script>

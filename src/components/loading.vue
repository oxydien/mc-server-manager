<style lang="scss" scoped>
@import "../assets/scss/global.scss";
$size: 60px;
$animation-duration: 1.3s;

.body {
  display: grid;
  place-items: center;
  position: relative;
}

.scene {
  position: relative;
  z-index: 2;
  height: $size * 2.75;
  width: $size * 2.75;
  display: grid;
  place-items: center;
}

.cube-wrapper {
  transform-style: preserve-3d;
  animation: bouncing $animation-duration infinite;
}

.cube {
  transform-style: preserve-3d;
  transform: rotateX(45deg) rotateZ(45deg);
  animation: rotation $animation-duration infinite;
}

.cube-faces {
  transform-style: preserve-3d;
  height: $size;
  width: $size;
  position: relative;
  transform-origin: 0 0;
  transform: translateX(0) translateY(0) translateZ(calc(($size - $size * 2) / 2));
}

.cube-face {
  position: absolute;
  inset: 0;
  background: $main-hm;
  border: solid 1px $main-h;
  &.shadow {
    background: $main-am;
    border: solid 1px $main-m;
    transform: translateZ(-$size);
    animation: bouncing-shadow $animation-duration infinite;
  }
  &.top {
    background: $main-h;
    transform: translateZ($size);
  }
  &.front {
    transform-origin: 0 50%;
    transform: rotateY(-90deg);
  }
  &.back {
    transform-origin: 0 50%;
    transform: rotateY(-90deg) translateZ(-$size);
  }
  &.right {
    transform-origin: 50% 0;
    transform: rotateX(-90deg) translateY(-$size);
  }
  &.left {
    transform-origin: 50% 0;
    transform: rotateX(-90deg) translateY(-$size) translateZ($size);
  }
}

@keyframes rotation {
  $start: 45deg;

  0% {
    transform: rotateX($start) rotateY(0) rotateZ($start);
    animation-timing-function: cubic-bezier(0.17, 0.84, 0.44, 1);
  }
  50% {
    transform: rotateX($start) rotateY(0) rotateZ($start + calc(360 / 2));
    animation-timing-function: cubic-bezier(0.76, 0.05, 0.86, 0.06);
  }
  100% {
    transform: rotateX($start) rotateY(0) rotateZ($start + 360);
    animation-timing-function: cubic-bezier(0.17, 0.84, 0.44, 1);
  }
}

@keyframes bouncing {
  0% {
    transform: translateY(-$size * 0.5);
    animation-timing-function: cubic-bezier(0.76, 0.05, 0.86, 0.06);
  }

  45% {
    transform: translateY($size * 0.5);
    animation-timing-function: cubic-bezier(0.23, 1, 0.32, 1);
  }

  100% {
    transform: translateY(-$size * 0.5);
    animation-timing-function: cubic-bezier(0.76, 0.05, 0.86, 0.06);
  }
}

@keyframes bouncing-shadow {
  0% {
    transform: translateZ(-$size) scale(1.3);
    animation-timing-function: cubic-bezier(0.76, 0.05, 0.86, 0.06);
    opacity: 0.05;
  }

  45% {
    transform: translateZ(0);
    animation-timing-function: cubic-bezier(0.23, 1, 0.32, 1);
    opacity: 0.3;
  }

  100% {
    transform: translateZ(-$size) scale(1.3);
    animation-timing-function: cubic-bezier(0.76, 0.05, 0.86, 0.06);
    opacity: 0.05;
  }
}
</style>

<template>
  <div class="body">
    <div class="scene">
      <div class="cube-wrapper">
        <div class="cube">
          <div class="cube-faces">
            <div class="cube-face shadow"></div>
            <div class="cube-face top"></div>
            <div class="cube-face left"></div>
            <div class="cube-face right"></div>
            <div class="cube-face back"></div>
            <div class="cube-face front"></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
// src: https://codepen.io/haja-ran/pen/xxWRKNm
export default {
  name: "LoadingCube",
};
</script>

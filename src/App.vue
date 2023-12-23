<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import Menu from "./components/menu.vue";
import NavBar from "./components/NavBar.vue";

// import Greet from "./components/Greet.vue";
</script>

<template>
  <NavBar />
  <div class="flex flex-row items-start mt-5">
    <Menu />
    <router-view></router-view>
  </div>
</template>

<style scoped></style>

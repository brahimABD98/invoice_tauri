<script setup>
import ListFacture from "../components/ListFacture.vue";
</script>

<template>
  <div class="mx-10 flex flex-row w-full">
    <div class="flex-col w-full">
      <router-link to="/invoice/new">
        <button class="btn btn-outline btn-accent mb-6">
          Ajouter une facture
        </button>
      </router-link>
      <ListFacture />
    </div>
  </div>
</template>

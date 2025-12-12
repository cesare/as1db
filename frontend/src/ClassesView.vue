<script setup lang="ts">
import { ref, onMounted } from "vue";
import type { Ref } from "vue";
import type { Class } from "./models";
import { fetchClasses } from "./api";

const classes: Ref<Class[]>= ref([]);

onMounted(async () => {
  const responseJson = await fetchClasses();
  classes.value = responseJson.classes;
});

</script>

<template>
  <h1>Classes</h1>
  <ul>
    <li v-for="clazz in classes" :key="clazz.id">
      <router-link :to="{ name: 'itemsOfClass', params: { id: clazz.id } }">{{ clazz.name }}</router-link>
    </li>
  </ul>
</template>

<style scoped></style>

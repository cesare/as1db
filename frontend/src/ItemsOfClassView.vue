<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import type { Ref } from "vue";
import { useRoute } from "vue-router";

interface Class {
  id: number,
  name: string,
}

interface Item {
  id: number,
  name: string,
}

interface Response {
  class: Class,
  items: Item[],
}

const clazz: Ref<Class | null> = ref(null);
const items: Ref<Item[]>= ref([]);

const route = useRoute()
type Parameter = typeof route.params.id

watch(() => route.params.id, fetchItems, { immediate: true })
async function fetchItems(id: Parameter) {
  if (id == null || Array.isArray(id)) return

  const response = await fetch(`http://localhost:3000/classes/${id}`);
  const responseJson: Response = await response.json();
  clazz.value = responseJson.class;
  items.value = responseJson.items;
}

</script>

<template>
  <h1>Class: {{ clazz?.name }}</h1>
  <ul>
    <li v-for="item in items" :key="item.id">
      {{ item.name }}
    </li>
  </ul>
</template>

<style scoped></style>

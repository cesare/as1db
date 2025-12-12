<script setup lang="ts">
import { ref, watch } from "vue";
import type { Ref } from "vue";
import { useRoute } from "vue-router";
import type { Class, Item } from "./models";
import { fetchItemsOfClass } from "./api";

const clazz: Ref<Class | null> = ref(null);
const items: Ref<Item[]>= ref([]);

const route = useRoute()
type Parameter = typeof route.params.id

watch(() => route.params.id, fetchItems, { immediate: true })
async function fetchItems(id: Parameter) {
  if (id == null || Array.isArray(id)) return

  const responseJson = await fetchItemsOfClass(id)
  clazz.value = responseJson.class;
  items.value = responseJson.items;
}

</script>

<template>
  <h1>Class: {{ clazz?.name }}</h1>
  <ul>
    <li v-for="item in items" :key="item.id">
      <RouterLink :to="{ name: 'itemDetails', params: { id: item.id } }">{{ item.name }}</RouterLink>
    </li>
  </ul>
</template>

<style scoped></style>

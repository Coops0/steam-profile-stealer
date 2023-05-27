<script setup lang="ts">
import { useCookieStore } from "@/stores/cookie";
import { ref } from "vue";
import { useLoadingStore } from "@/stores/loading";

const emit = defineEmits<{
  (event: 'save-cookie'): void
}>();

const cookieInput = ref('');
const cookieStore = useCookieStore();
const loadingStore = useLoadingStore();

if (localStorage.getItem('cookie')) {
  cookieInput.value = localStorage.getItem('cookie')!;
  saveCookie();
  loadingStore.loading = true;
}

function saveCookie() {
  localStorage.setItem('cookie', cookieInput.value);
  cookieStore.cookie = cookieInput.value;
  emit('save-cookie');
}
</script>

<template>
  <v-text-field
      label="Steam Login Secure Cookie"
      v-model="cookieInput"
      :disabled="loadingStore.loading"
  ></v-text-field>
  <v-btn @click="saveCookie" :disabled="loadingStore.loading">Save</v-btn>
</template>
<style scoped>

</style>
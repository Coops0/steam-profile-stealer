<script setup lang="ts">
import { useCookieStore } from "@/stores/cookie";
import { ref } from "vue";

const emit = defineEmits<{
  (event: 'saveCookie'): void
}>();

const cookieInput = ref('');
const cookieStore = useCookieStore();

if (localStorage.getItem('cookie')) {
  cookieInput.value = localStorage.getItem('cookie')!;
  saveCookie();
}

function saveCookie() {
  localStorage.setItem('cookie', cookieInput.value);
  cookieStore.cookie = cookieInput.value;
  emit('saveCookie');
}
</script>

<template>
  <v-text-field
      label="Steam Login Secure Cookie"
      v-model="cookieInput"
  ></v-text-field>
  <v-btn @click="saveCookie">Save</v-btn>
</template>
<style scoped>

</style>
<script setup lang="ts">
import { useMessageStore } from "@/stores/messages";

const messageStore = useMessageStore();
</script>

<template>
  <p v-for="message in messageStore.messages" :key="message.key" :class="{'text-red': message.error}">
    {{ message.message }}
  </p>
</template>

<style scoped>
/*noinspection CssOverwrittenProperties*/
p {
  font-family: 'Roboto', sans-serif;
  font-family: 'Roboto Mono', monospace;
}
</style>
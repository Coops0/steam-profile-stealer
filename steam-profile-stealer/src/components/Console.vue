<script setup lang="ts">
import { useMessageStore } from "@/stores/messages";

const messageStore = useMessageStore();
</script>

<template>
  <v-slide-y-transition group tag="v-list">
    <template v-for="(message, i) in messageStore.messages" :key="message.key">
      <v-divider v-if="i !== 0" :key="`div-${message.key}`"></v-divider>
      <v-list-item>
        <v-label
            :class="{
              'text-red': message.error,
              'text-green': message.message.toLowerCase().includes('success'),
              'text-yellow': i === 0 && !message.error
        }"
            v-text="message.message"
        >
        </v-label>
      </v-list-item>
    </template>
  </v-slide-y-transition>
</template>
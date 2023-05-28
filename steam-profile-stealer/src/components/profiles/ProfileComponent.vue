<script setup lang="ts">
import type { Profile } from "@/stores/profile";

const profile = defineProps<Profile>();
</script>

<template>
  <v-card>
    <v-img
        :src="profile.image_url"
        cover
    ></v-img>

    <v-card-title>
      {{ profile.name.trim().length === 0 ? '(no name)' : profile.name }}
    </v-card-title>

    <v-card-actions>
      <v-btn
          :href="profile.url"
          target="_blank"
          variant="text"
          color="deep-purple"
          append-icon="mdi-open-in-new"
      >
        Link
      </v-btn>
      <slot></slot>
    </v-card-actions>

  </v-card>
</template>
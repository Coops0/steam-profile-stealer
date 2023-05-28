<script setup lang="ts">
import ProfileComponent from "@/components/profiles/ProfileComponent.vue";
import { useProfileStore } from "@/stores/profile";
import { useLoadingStore } from "@/stores/loading";

const profileStore = useProfileStore();
const loadingStore = useLoadingStore();

defineEmits<{
  (event: 'refresh-profile'): void
  (event: 'steal-profile'): void
}>();

</script>

<template>
  <v-col cols="6">
    <profile-component v-bind="profileStore.selfProfile!" class="pa-3 ma-3">
      <v-btn
          :disabled="loadingStore.loading"
          @click="$emit('refresh-profile')"
          class="pa-3 ma-3"
          variant="tonal"
          color="teal-accent-4"
          append-icon="mdi-refresh"
      >Refresh
      </v-btn>
    </profile-component>
  </v-col>

  <v-col cols="6" v-if="profileStore.targetProfile">
    <profile-component v-bind="profileStore.targetProfile!" class="pa-3 ma-3">
      <v-btn
          :disabled="loadingStore.loading"
          @click="$emit('steal-profile')"
          class="pa-3 ma-3"
          color="red"
          variant="tonal"
          append-icon="mdi-import"
      >Steal
      </v-btn>
    </profile-component>
  </v-col>
</template>
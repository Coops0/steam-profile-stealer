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
  <v-container>
    <v-row>
      <v-col>
        <v-text-field
            label="Steam Login Secure Cookie"
            v-model="cookieInput"
            :disabled="loadingStore.loading"
            :loading="loadingStore.loading"
            @keydown.enter="saveCookie"
        ></v-text-field>
      </v-col>
      <v-col>
        <v-btn
            @click="saveCookie"
            :disabled="loadingStore.loading"
            :loading="loadingStore.loading"
            variant="tonal"
            color="green"
            append-icon="mdi-upload"
        >Save</v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-btn
            variant="flat"
            target="_blank"
            href="https://youtu.be/PzWTrs152wY"
            color="blue"
            append-icon="mdi-open-in-new"
        >What is this?
        </v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>
<style scoped>

</style>
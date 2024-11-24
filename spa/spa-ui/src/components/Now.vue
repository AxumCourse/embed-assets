<script setup lang="ts">
import dayjs from "dayjs";
import { onMounted, ref } from "vue";

const remoteNow = ref<string>("正在获取");
const loadData = () => {
  fetch("/now", {
    method: "GET",
  })
    .then((r) => r.text())
    .then((r) => (remoteNow.value = r));
};

onMounted(() => {
  loadData();
});
</script>

<template>
  <ul>
    <li>服务器时间：{{ remoteNow }}</li>
    <li>客户端时间：{{ dayjs().format("YYYY-MM-DD HH:mm:ss") }}</li>
  </ul>
</template>

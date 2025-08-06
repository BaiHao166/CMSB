<script setup>
import List from "./List.vue"

const columns = ["name", "size", "date", "status", "action"];

let datasources = [
    { name: "1.mp4", size: "1.2M", date: "2023-05-05", status: "上传中", action: "暂停" },
    { name: "2.mp4", size: "1.2M", date: "2023-05-05", status: "上传中", action: "暂停" },
    { name: "3.mp4", size: "1.2M", date: "2023-05-05", status: "上传中", action: "暂停" },
    { name: "4.mp4", size: "1.2M", date: "2023-05-05", status: "上传中", action: "暂停" }
];

const widths = ["40%", "15%", "15%", "15%", "15%"]
const textAlign = ["left", "center", "center", "center", "center"]
</script>

<template>
    <div>
        <List :columns="columns" :datasources="datasources" :widths="widths" :text-align="textAlign"></List>
    </div>
</template>

<style scoped></style>
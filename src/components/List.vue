<script setup>
import { defineProps, computed, onMounted, nextTick } from "vue"

const props = defineProps({
    columns: {
        type: Array,
        required: true,
        default: []
    },
    widths: {
        type: Array,
        required: false,
        default: []
    },
    textAlign: {
        type: Array,
        required: false,
        default: []
    },
    datasources: {
        type: Array,
        required: true,
        default: []
    },
    pageInfo: {
        type: Object,
        required: false,
        default: {
            pageNo: 1,
            pageSize: 10,
            total: 100
        }
    }
})

let pageButtons = computed(() => {
    let length = props.pageInfo.total / props.pageInfo.pageSize;
    let array = [];
    for (let i = 0; i < length; i++) {
        array.push(i + 1);
    }

    return array;
});

let tdWidths = computed(() => {
    if (props.widths.length > 0) {
        return props.widths;
    }

    return props.columns.map(() => "auto");
});

let tdTextAlign = computed(() => {
    if (props.textAlign.length > 0) {
        return props.textAlign;
    }

    return props.columns.map(() => "center");
});

function clickPage(pageNo, event) {
    let nodes = document.querySelectorAll(".page-active");
    if (nodes && nodes.length > 0) {
        nodes.forEach(node => {
            node.classList.remove("page-active");
            node.classList.add("page-seri");
        });
    }

    event.target.classList.remove("page-seri");
    event.target.classList.add("page-active");
}

function frontPage() {
    
}

function nextPage() {

}

onMounted(() => {
    let nodes = document.querySelectorAll(".page-seri");
    if (nodes) {
        nodes[1].click();
    }
})



</script>

<template>
    <div class="table-wrapper">
        <div class="table-container">
            <table>
                <thead>
                    <tr>
                        <th v-for="(column, index) in props.columns" :key="index" :style="{ width: tdWidths[index] }">
                            {{ column }}
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="(data, index) in props.datasources" :key="index">
                        <td v-for="(field, index) in Object.keys(data)" :key="index"
                            :style="{ textAlign: tdTextAlign[index] }">
                            {{ data[field] }}
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="page">
            <ul>
                <li class="page-seri">上一页</li>
                <li class="page-seri" v-for="num in pageButtons" :key="num" @click="clickPage(num, $event)">
                    {{ num }}
                </li>
                <li class="page-seri">下一页</li>
            </ul>
        </div>
    </div>
</template>

<style scoped>
.table-wrapper {
    width: 100%;
    height: 100%;
    .page {
       margin-top: 15px;
    }
}

.table-wrapper .page ul {
    list-style: none;
    padding: 0px;
    margin: 0px;
    display: flex;
    justify-content: flex-end;
    align-items: center;
}
.table-wrapper .page ul li {
    padding: 5px 12px;
    border-radius: 5px;
    border: 1px solid white;
    margin-right: 0;
}
.table-wrapper .page ul li:hover {
    cursor: pointer;
}

.table-wrapper .page ul .page-seri:hover {
    background-color: #f2f1f1;
    border: 1px solid #f2f1f1;
}

.table-wrapper .page ul .page-active {
    border: 1px solid #1677ff;
    color: #1677ff;
}

table {
    width: 100%;
    border-collapse: collapse;

    td {
        padding: 0 10px;
    }
}

table tr {
    height: 6vh;
    border-bottom: 1px solid #f1f1f1;
}

table>tbody>tr:hover {
    background-color: #f2f1f1;
}

table>thead>tr {
    border-radius: 10px 10px 0 0;
}

table th {
    text-align: center;
    background-color: #fafafa;
}
</style>
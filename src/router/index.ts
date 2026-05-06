import { createRouter, createWebHistory } from 'vue-router'
import Analyzer from '../views/Analyzer.vue'
import TagManager from '../views/TagManager.vue'
import AgentManager from '../views/AgentManager.vue'

const routes = [
    { path: '/', name: 'Analyzer', component: Analyzer },
    { path: '/tags', name: 'Tags', component: TagManager },
    { path: '/agents', name: 'Agents', component: AgentManager },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router
import { Hono } from 'hono'
import { handle } from 'hono/vercel'

export const config = { runtime: 'edge' }

const app = new Hono()

app.get('/hello', (c) => c.json({ message: 'Hello World!' }))

export default handle(app)

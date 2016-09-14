import Koa from 'koa'
import views from 'koa-views'
import bodyparser from 'koa-bodyparser'
import logger from 'koa-logger'

const app = new Koa()

app
  .use(bodyparser())
  .use(logger())

app.listen(9999, () => console.log('http://localhost:9999'))
'use client'

import { Button, Card, Title, Pagination } from '@mantine/core'
import { useEffect, useState } from 'react'
import { Page } from '@/components/Page'
import { Topic, fetchTopics } from '@/app/store'
import classes from './page.module.css'

const pageSize = 10

export default function GET() {
  const [activePage, setPage] = useState(1)
  const [topics, setTopics] = useState<Topic[]>([])
  const [topicCount, setTopicCount] = useState<number>(0)

  useEffect(() => {
    (async function thunk() {
      const { total, items } = await fetchTopics()
      setTopicCount(total)
      setTopics(items.slice(0, pageSize))
    }())
  }, [setTopics])

  const pageCount = topicCount / pageSize

  return (
    <Page>
      <div className={classes.top}>
        <Title className={classes.title} order={2}>Topics</Title>

        <Button
          color="green"
          variant="filled"
          component="a"
          href="/topics/new"
          className={classes.addButton}
          >
            Add
        </Button>
      </div>

      <div className={classes.results}>
        {
          topics.map(({ id, name }) => (
            <Card
              key={id}
              component="a"
              href={`/topics/${id}`}
              padding="sm"
              radius="md"
              className={classes.topicCard}>
              {name}
            </Card>
          ))
        }
      </div>

      <Pagination total={pageCount} value={activePage} onChange={setPage} my="sm" />
    </Page>
  )
}

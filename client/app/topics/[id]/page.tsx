'use client'

import { useEffect, useState } from 'react'
import { Title } from '@mantine/core'
import { Topic, fetchTopic } from '@/lib/store'
import { Page } from '@/components/Page'

const page = (name: string, inner: React.ReactNode): React.ReactNode => (
  <Page>
    <Title order={3}>
      {name}
    </Title>

    {inner}
  </Page>
)

export default function GET({ params }: { params: { id: string } }) {
  const [topic, setTopic] = useState<Topic | null>(null)
  const [requestInFlight, setRequestInFlight] = useState(true)
  const { id: topicId } = params

  useEffect(() => {
    async function thunk() {
      const res = await fetchTopic(topicId)
      setTopic(res.topic)
      setRequestInFlight(false)
      console.log('request completed.')
    }

    thunk()
  }, [setTopic, setRequestInFlight, topicId])

  if (topic == null) {
    if (requestInFlight) {
      return page('Fetching topic', <p>Waiting to hear back from server...</p>)
    }
      return page('Not found', <p>Topic {topicId} was not found.</p>)
  }

  const { name } = topic

  return page(name,
    <p>
      Details about the topic include ...
    </p>
  )
}

import { Title, Text } from '@mantine/core'
import { Page } from '@/components/Page'

export default function GET() {
  return (
    <Page>
      <Title order={2} mb={15}>Workspaces</Title>
      <Text>
        Independent workspaces that allow you to work with different configurations and options,
        depending on what you want to do.
      </Text>
    </Page>
  )
}

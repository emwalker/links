'use client'

import { Box, Button, Group, TextInput, Title } from '@mantine/core'
import { UseFormReturnType, useForm } from '@mantine/form'
import { useRouter } from 'next/navigation'
import { AppRouterInstance } from 'next/dist/shared/lib/app-router-context.shared-runtime'
import { notifications } from '@mantine/notifications'
import { Page } from '@/components/Page'
import { ROOT_USER_ID, createTopic } from '@/lib/store'
import classes from './page.module.css'

type Errors = {
  [key: string]: string
}

function submitForm(router: AppRouterInstance, form: UseFormReturnType<any>) {
  return form.onSubmit(async (payload) => {
    const res = await createTopic(payload)
    const errorFields = Object.keys(res.errors)

    if (errorFields.length > 0) {
      const errors: Errors = {}

      errorFields.forEach((memo, fieldIndex) => {
        const field = errorFields[fieldIndex]
        const [first] = res.errors[field] || []
        if (first) {
          errors[field] = first
        }
        return memo
      }, {})

      form.setErrors(errors)
      return
    }

    const { name } = payload

    console.log('topic added: ', res)
    router.push('/topics')
    notifications.show({
      title: 'Topic added',
      message: `A new topic has been added: ${name}`,
      color: 'blue',
      autoClose: 8000,
    })
  })
}

export default function GET() {
  const router = useRouter()

  const form = useForm({
    mode: 'uncontrolled',
    initialValues: {
      name: '',
      owner_id: ROOT_USER_ID,
    },

    validate: {
      name: (value) => value ? null : 'A name is required',
    },
  })

  return (
    <Page>
      <Title order={3}>Add a topic</Title>

      <Box maw={700} className={classes.form}>
        <form onSubmit={submitForm(router, form)}>
          <TextInput
            withAsterisk
            label="Name"
            placeholder="Name of topic"
            key={form.key('name')}
            {...form.getInputProps('name')}
          />

          <Group justify="flex-start" mt="md">
            <Button variant="filled" color="green" type="submit">Add</Button>
          </Group>
        </form>
      </Box>
    </Page>
  )
}

import { buildName as makeName } from './shared.bicep'
import * as shared from './shared.bicep'
import 'sys@1.0.0'

targetScope = 'resourceGroup'

@description('Storage account name')
param storageAccountName string

param baseTags object = {
  environment: 'dev'
}

var extraTags = {
  service: 'app'
}

var mergedTags = {
  ...baseTags
  ...extraTags
}

var numbers = [1, 2, 3]
var lastNumber = numbers[^1]
var maybeLastNumber = numbers[?^1]
var positiveCount = +length(numbers)

resource stg 'Microsoft.Storage/storageAccounts@2023-05-01' = {
  name: makeName(storageAccountName)
  location: resourceGroup().location
  sku: {
    name: 'Standard_LRS'
  }
  kind: 'StorageV2'
  tags: mergedTags
}

output inputResource resourceInput<'Microsoft.Storage/storageAccounts@2023-05-01'> = stg
output outputResource resourceOutput<'Microsoft.Storage/storageAccounts@2023-05-01'> = stg
output summary object = {
  imported: shared
  count: positiveCount
}

// Code generated by MockGen. DO NOT EDIT.
// Source: pkg/providers/etcd.go
//
// Generated by this command:
//
//	mockgen -source pkg/providers/etcd.go -destination pkg/providers/mock_providers/etcd_mock.go
//

// Package mock_providers is a generated GoMock package.
package mock_providers

import (
	context "context"
	reflect "reflect"

	v3 "go.etcd.io/etcd/client/v3"
	gomock "go.uber.org/mock/gomock"
)

// MockEtcdClient is a mock of EtcdClient interface.
type MockEtcdClient struct {
	ctrl     *gomock.Controller
	recorder *MockEtcdClientMockRecorder
}

// MockEtcdClientMockRecorder is the mock recorder for MockEtcdClient.
type MockEtcdClientMockRecorder struct {
	mock *MockEtcdClient
}

// NewMockEtcdClient creates a new mock instance.
func NewMockEtcdClient(ctrl *gomock.Controller) *MockEtcdClient {
	mock := &MockEtcdClient{ctrl: ctrl}
	mock.recorder = &MockEtcdClientMockRecorder{mock}
	return mock
}

// EXPECT returns an object that allows the caller to indicate expected use.
func (m *MockEtcdClient) EXPECT() *MockEtcdClientMockRecorder {
	return m.recorder
}

// Get mocks base method.
func (m *MockEtcdClient) Get(ctx context.Context, key string, opts ...v3.OpOption) (*v3.GetResponse, error) {
	m.ctrl.T.Helper()
	varargs := []any{ctx, key}
	for _, a := range opts {
		varargs = append(varargs, a)
	}
	ret := m.ctrl.Call(m, "Get", varargs...)
	ret0, _ := ret[0].(*v3.GetResponse)
	ret1, _ := ret[1].(error)
	return ret0, ret1
}

// Get indicates an expected call of Get.
func (mr *MockEtcdClientMockRecorder) Get(ctx, key any, opts ...any) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	varargs := append([]any{ctx, key}, opts...)
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "Get", reflect.TypeOf((*MockEtcdClient)(nil).Get), varargs...)
}

// Put mocks base method.
func (m *MockEtcdClient) Put(ctx context.Context, key, val string, opts ...v3.OpOption) (*v3.PutResponse, error) {
	m.ctrl.T.Helper()
	varargs := []any{ctx, key, val}
	for _, a := range opts {
		varargs = append(varargs, a)
	}
	ret := m.ctrl.Call(m, "Put", varargs...)
	ret0, _ := ret[0].(*v3.PutResponse)
	ret1, _ := ret[1].(error)
	return ret0, ret1
}

// Put indicates an expected call of Put.
func (mr *MockEtcdClientMockRecorder) Put(ctx, key, val any, opts ...any) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	varargs := append([]any{ctx, key, val}, opts...)
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "Put", reflect.TypeOf((*MockEtcdClient)(nil).Put), varargs...)
}
